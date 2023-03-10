use std::{convert::Infallible, sync::Arc};

use ethers::{
    abi::{AbiEncode, Tokenize},
    prelude::{builders::ContractCall, Address, Bytes, Signature, U256},
    providers::Middleware,
    signers::Signer,
    types::{
        transaction::{
            eip2718::TypedTransaction,
            eip712::{EIP712Domain, Eip712},
        },
        Eip1559TransactionRequest, SignatureError, H256, I256,
    },
    utils::keccak256,
};

use crate::{bindings::mev_wallet_v0::MevTxCall, MevWalletV1};

/// A MEV-driven Meta-transaction. MEV Transactions are intended to be used
/// with a [`MevWalletV0`] smart contract. They describe a transaction initiated
/// by that contract, similar to a Gnosis Safe.
///
/// Unlike a Safe, a `MevTx` does not implement a multisig. Instead, it allows
/// a single signer to specify an explicit tip in `MevWeth`. This `tip` can be
/// positive OR negative. A negative tip specifies that the user wants to
/// receive some MEV from the transaction.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MevTx {
    /// The chain id on which the wallet is deployed
    pub chain_id: u64,
    /// Address of the wallet from which this will be sent
    pub wallet: Address,
    /// Target
    pub to: Address,
    /// Data
    pub data: Bytes,
    /// Value to forward
    pub value: I256,
    /// True for delegatecall
    pub delegate: bool,
    /// MevWeth Tip amount
    pub tip: I256,
    /// MaxBaseFee (0 for none)
    pub max_base_fee: U256,
    /// notBefore and deadline
    pub timing: U256,
    /// Nonce
    pub nonce: U256,
}

/// EIP-712 calculation info for MevTx
pub struct MevTx712<'a> {
    wallet: Address,
    chain_id: u64,
    tx: &'a MevTx,
}

impl<'a> MevTx712<'a> {
    /// Instnatiate a new Eip-712 struct
    pub fn new(wallet: Address, chain_id: u64, tx: &'a MevTx) -> Self {
        Self {
            wallet,
            chain_id,
            tx,
        }
    }
}

impl<'a> Eip712 for MevTx712<'a> {
    type Error = Infallible;

    fn domain(&self) -> Result<ethers::types::transaction::eip712::EIP712Domain, Self::Error> {
        Ok(EIP712Domain {
            name: Some("MevTx".to_owned()),
            version: Some("1".to_owned()),
            chain_id: Some(self.chain_id.into()),
            verifying_contract: Some(self.wallet),
            salt: None,
        })
    }

    fn type_hash() -> Result<[u8; 32], Self::Error> {
        Ok(crate::TX_TYPEHASH)
    }

    fn struct_hash(&self) -> Result<[u8; 32], Self::Error> {
        Ok(keccak256(self.tx.encode_struct()))
    }

    fn encode_eip712(&self) -> Result<[u8; 32], Self::Error> {
        // encode the digest to be compatible with solidity abi.encodePacked()
        // See: https://github.com/gakonst/ethers-rs/blob/master/examples/permit_hash.rs#L72

        let domain_separator = self.domain_separator()?;
        let struct_hash = self.struct_hash()?;

        let digest_input = [&[0x19, 0x01], &domain_separator[..], &struct_hash[..]].concat();

        Ok(keccak256(digest_input))
    }
}

impl Tokenize for &MevTx {
    fn into_tokens(self) -> Vec<ethers::abi::Token> {
        (
            crate::TX_TYPEHASH,
            self.to,
            H256::from(keccak256(&self.data)),
            self.value,
            self.delegate,
            self.tip,
            self.max_base_fee,
            self.timing,
            self.nonce,
        )
            .into_tokens()
    }
}

impl MevTx {
    /// Encode the preimage of Eip-712's hashStruct
    pub fn encode_struct(&self) -> Vec<u8> {
        ethers::abi::encode(&self.into_tokens())
    }

    /// Return an eip-712 struct referencing this tx, suitable for signing or
    /// recovering signatures
    pub fn eip_712(&self) -> MevTx712<'_> {
        MevTx712 {
            wallet: self.wallet,
            chain_id: self.chain_id,
            tx: self,
        }
    }

    /// Sign the MEV transaction with the provided signer.
    pub async fn sign<S: Signer>(self, signer: &S) -> Result<SignedMevTx, S::Error> {
        let eip712 = self.eip_712();
        let sig = signer.sign_typed_data(&eip712).await?;
        Ok(SignedMevTx { tx: self, sig })
    }
}

/// A Signed MEV Tx
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SignedMevTx {
    #[serde(flatten)]
    tx: MevTx,
    #[serde(flatten)]
    sig: Signature,
}

impl SignedMevTx {
    /// Convert into a populated TypedTransaction, ready for population and
    /// dispatch
    pub fn wrap_in_tx(self) -> TypedTransaction {
        let mut r = [0u8; 32];
        let mut s = [0u8; 32];
        self.sig.r.to_big_endian(&mut r);
        self.sig.s.to_big_endian(&mut s);

        let data = MevTxCall {
            to: self.tx.to,
            data: self.tx.data,
            value: self.tx.value,
            delegate: self.tx.delegate,
            tip: self.tx.tip,
            max_base_fee: self.tx.max_base_fee,
            timing: self.tx.timing,
            n: self.tx.nonce,
            v: self.sig.v as u8,
            r,
            s,
        }
        .encode();

        TypedTransaction::Eip1559(Eip1559TransactionRequest {
            from: None,
            to: Some(self.tx.wallet.into()),
            gas: None,
            value: None,
            data: Some(data.into()),
            nonce: None,
            access_list: Default::default(),
            max_priority_fee_per_gas: Some(U256::zero()),
            max_fee_per_gas: None,
            chain_id: Some(self.tx.chain_id.into()),
        })
    }

    /// Convert the Signed MEV tx into a call to the contract wallet
    pub fn into_call<M: Middleware>(self, middleware: Arc<M>) -> ContractCall<M, ()> {
        let contract = self.wallet_contract(middleware);
        let mut r = [0u8; 32];
        let mut s = [0u8; 32];
        self.sig.r.to_big_endian(&mut r);
        self.sig.s.to_big_endian(&mut s);

        let mut call = contract.mev_tx(
            self.tx.to,
            self.tx.data,
            self.tx.value,
            self.tx.delegate,
            self.tx.tip,
            self.tx.max_base_fee,
            self.tx.timing,
            self.tx.nonce,
            self.sig.v as u8,
            r,
            s,
        );

        let req = call.tx.as_eip1559_mut().expect("no legacy tx");
        if !self.tx.max_base_fee.is_zero() {
            req.max_fee_per_gas = Some(self.tx.max_base_fee);
        }
        req.max_priority_fee_per_gas = Some(U256::zero());

        call
    }

    /// Get the chain ID
    pub fn chain_id(&self) -> u64 {
        self.tx.chain_id
    }

    /// Get the Wallet address
    pub fn wallet(&self) -> Address {
        self.tx.wallet
    }

    /// Return an instance of the wallet contract
    pub fn wallet_contract<M: Middleware>(&self, middleware: Arc<M>) -> MevWalletV1<M> {
        MevWalletV1::new(self.tx.wallet, middleware)
    }

    /// Get the transaction details
    pub fn tx(&self) -> &MevTx {
        &self.tx
    }

    /// Get the signature
    pub fn sig(&self) -> Signature {
        self.sig
    }

    /// Get the signer
    pub fn signer(&self) -> Result<Address, SignatureError> {
        self.sig.recover_typed_data(self.tx.eip_712())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn it_generates_json_output() {
        let tx = SignedMevTx {
            tx: MevTx {
                chain_id: 31337,
                wallet: Address::zero(),
                to: Address::zero(),
                data: "0x1234abcd".parse().unwrap(),
                value: 500.into(),
                delegate: true,
                tip: 9000.into(),
                max_base_fee: 83.into(),
                timing: 123845.into(),
                nonce: 16.into(),
            },
            sig: Signature {
                r: U256::from(851929865),
                s: U256::from(1273717),
                v: 18,
            },
        };
        println!("{}", serde_json::to_string_pretty(&tx).unwrap());
    }
}
