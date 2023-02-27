use async_trait::async_trait;

use ethers::{
    prelude::ContractError,
    providers::{Middleware, PendingTransaction, ProviderError},
    signers::Signer,
    types::{Address, U256},
};
use std::sync::Arc;

use crate::{
    bindings::{ierc20::IERC20, mev_wallet_v1::MevWalletV1Errors},
    deploy::{deploy_proxy, deploy_proxy_with_owner},
    MevTx, MevWalletV1, SignedMevTx, MEV_WETH_ADDR,
};

use super::{BuilderError, MevTxBuilderInternal};

impl<M> crate::MevWalletV1<M>
where
    M: Middleware + 'static,
{
    /// Deploy a new proxy wallet
    /// Returns a new contract once deployment has succeeded
    pub async fn new_proxy(
        client: Arc<M>,
        salt: impl Into<[u8; 32]>,
    ) -> Result<Self, ContractError<M>>
    where
        M: 'static,
    {
        let call = deploy_proxy(client.clone(), salt);
        let address = call.clone().await?;

        let receipt = call
            .send()
            .await?
            .await
            .map_err(M::convert_err)
            .map_err(ContractError::from_middleware_error)?;
        if receipt.is_none() {
            return Err(
                ProviderError::CustomError("Could not get deploy tx receipt".to_owned()).into(),
            );
        }
        Ok(MevWalletV1::new(address, client))
    }

    /// Deploy a new proxy wallet with a specified owner
    /// Returns a new contract once deployment has succeeded
    pub async fn new_proxy_with_owner(
        client: Arc<M>,
        salt: impl Into<[u8; 32]>,
        owner: impl Into<Address>,
    ) -> Result<Self, ContractError<M>>
    where
        M: 'static,
    {
        let call = deploy_proxy_with_owner(client.clone(), salt, owner);
        let address = call.clone().await?;

        let receipt = call
            .send()
            .await?
            .await
            .map_err(M::convert_err)
            .map_err(ContractError::from_middleware_error)?;
        if receipt.is_none() {
            return Err(
                ProviderError::CustomError("Could not get deploy tx receipt".to_owned()).into(),
            );
        }
        Ok(MevWalletV1::new(address, client))
    }

    /// Create a [`MevTxBuilder`] from an ethers `TypedTransaction`. This
    /// convenience method allows users to convert regular contract
    /// interactions into MEV txns easily.
    pub async fn convert_tx(
        &self,
        typed_tx: &ethers::types::transaction::eip2718::TypedTransaction,
    ) -> Result<MevTxBuilderInternal<MevWalletV1<M>>, BuilderError> {
        MevTxBuilderInternal::from_typed_tx(self, typed_tx).await
    }

    /// Create a contract call object sending the signed mev transaction
    pub fn send(&self, tx: SignedMevTx) -> ethers::prelude::builders::ContractCall<M, ()> {
        tx.into_call(self.client())
    }

    /// True if the input `address` is the owner of this MevWallet, false
    /// otherwise
    pub async fn is_owner(&self, address: Address) -> Result<bool, ContractError<M>> {
        Ok(self.owner().await? == address)
    }

    /// Checks the `MevWeth` balance of this MevWallet
    pub async fn mev_weth_balance(&self) -> Result<U256, ContractError<M>> {
        let mware = self.client();
        IERC20::new(MEV_WETH_ADDR, mware)
            .balance_of(self.address())
            .await
    }

    /// Return the ETH balance of this MevWallet
    pub async fn balance(&self) -> Result<U256, ContractError<M>> {
        self.client()
            .get_balance(self.address(), None)
            .await
            .map_err(ContractError::from_middleware_error)
    }

    /// Simulate a tx. Returns `Ok(None)` for succesful execution. Returns Ok
    /// (Some(err)) for contract revert. Returns Err(e) for error during
    /// simulation
    ///
    pub async fn simulate(
        &self,
        tx: SignedMevTx,
    ) -> Result<Option<MevWalletV1Errors>, ContractError<M>> {
        let call = tx.into_call(self.client());

        let result = call.await;

        // execution succeeded
        if result.is_ok() {
            return Ok(None);
        }
        let err = result.expect_err("checked by is_ok");

        if let Some(mev_err) = err.decode_contract_revert() {
            Ok(Some(mev_err))
        } else {
            Err(err)
        }
    }
}

#[async_trait]
/// Extension trait for ethers middleware. Allows sending MevTxns directly
pub trait MiddlewareExt: Middleware {
    /// Wrap a MevTx in a transaction and dispatch it
    async fn send_mev_tx(
        &self,
        mev_tx: SignedMevTx,
    ) -> Result<PendingTransaction<'_, Self::Provider>, Self::Error> {
        let mut tx = mev_tx.wrap_in_tx();

        // Populate max fee per gas :)
        let (mfpg, _) = self.estimate_eip1559_fees(None).await?;
        tx.as_eip1559_mut().unwrap().max_fee_per_gas = Some(mfpg);

        // resolve nonce & gas & such
        self.fill_transaction(&mut tx, None).await?;

        // delegate to inner provider, basically :)
        self.send_transaction(tx, None).await
    }
}

/// A signer extension for signing MevTxns
#[async_trait]
pub trait SignerExt: Signer + Sized {
    /// Sign a mev transaction
    async fn sign_mev_tx(&self, mev_tx: MevTx) -> Result<SignedMevTx, Self::Error> {
        mev_tx.sign(self).await
    }
}
