use ethers::{
    prelude::{Address, Bytes, EthCall, U256},
    providers::Middleware,
    signers::Signer,
    types::{transaction::eip2718::TypedTransaction, Eip1559TransactionRequest, I256},
};

use crate::{delegate_builder, delegate_builder_populate, MevTx, MevWalletV1, SignedMevTx};

/// Error type for [`MevTxBuilder`]
#[derive(Debug, thiserror::Error)]
pub enum BuilderError {
    /// Missing keys
    #[error("Missing Keys: {0:?}")]
    MissingKeys(Vec<&'static str>),
    /// From wallet is not set.
    #[error("Missing Contract. Set `wallet()` on the builder")]
    MissingContract,
    /// Signer has an unexpected chain id
    #[error("Signer has an unexpected chain id. Tx has {tx}, signer has {signer}")]
    ChainIdMismatch {
        /// Transaction's chain id
        tx: u64,
        /// Signer's chain id
        signer: u64,
    },
    /// Custom error (e.g. signer)
    #[error("(0)")]
    Custom(String),
}

/// The `MevTxBuilder` builds MevTxns.
pub type MevTxBuilder = MevTxBuilderInternal<()>;

/// The `SignedMevTxBuilder` builds MevTxns.
pub type SignedMevTxBuilder<'a, S> = SignedMevTxBuilderInternal<'a, (), S>;

/// A Builder for `MevTx`
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MevTxBuilderInternal<M> {
    wallet: Option<M>,
    chain_id: Option<u64>,
    to: Option<Address>,
    data: Option<Bytes>,
    value: Option<I256>,
    delegate: Option<bool>,
    tip: Option<I256>,
    max_base_fee: Option<U256>,
    deadline: Option<U256>,
    not_before: Option<U256>,
    nonce: Option<U256>,
}

#[allow(clippy::derivable_impls)]
impl Default for MevTxBuilder {
    fn default() -> Self {
        Self {
            chain_id: Default::default(),
            wallet: Default::default(),
            to: Default::default(),
            data: Default::default(),
            value: Default::default(),
            delegate: Default::default(),
            tip: Default::default(),
            max_base_fee: Default::default(),
            deadline: Default::default(),
            not_before: Default::default(),
            nonce: Default::default(),
        }
    }
}

impl<M> From<&MevTxBuilderInternal<M>> for TypedTransaction {
    fn from(val: &MevTxBuilderInternal<M>) -> Self {
        TypedTransaction::Eip1559(Eip1559TransactionRequest {
            from: None,
            to: val.to.map(Into::into),
            value: val.value.map(I256::into_raw),
            data: val.data.clone(),
            nonce: None,
            access_list: Default::default(),
            max_priority_fee_per_gas: None,
            max_fee_per_gas: val.max_base_fee,
            chain_id: None,
            gas: None,
        })
    }
}

impl MevTxBuilderInternal<()> {
    /// Instantiate an empty builder
    pub fn new() -> Self {
        Default::default()
    }
}

impl<C> MevTxBuilderInternal<C> {
    /// Set the MevWallet from which this tx will be sent
    pub fn wallet<M>(self, contract: &MevWalletV1<M>) -> MevTxBuilderInternal<MevWalletV1<M>>
    where
        M: Middleware + 'static,
    {
        MevTxBuilderInternal {
            chain_id: self.chain_id,
            wallet: Some(contract.clone()),
            to: self.to,
            data: self.data,
            value: self.value,
            delegate: self.delegate,
            tip: self.tip,
            max_base_fee: self.max_base_fee,
            deadline: self.deadline,
            not_before: self.not_before,
            nonce: self.nonce,
        }
    }

    /// Set `to`
    pub fn to(mut self, to: Address) -> Self {
        self.to = Some(to);
        self
    }

    /// Set `data`
    pub fn data(mut self, data: impl Into<Bytes>) -> Self {
        self.data = Some(data.into());
        self
    }

    /// Set `data` to any contract call
    pub fn call<E>(mut self, call: E) -> Self
    where
        E: EthCall,
    {
        self.data = Some(call.encode().into());
        self
    }

    /// Set `value`
    pub fn value(mut self, value: impl Into<I256>) -> Self {
        self.value = Some(value.into());
        self
    }

    /// Set `delegate`
    pub fn delegate(mut self, delegate: bool) -> Self {
        self.delegate = Some(delegate);
        self
    }

    /// Set `tip`
    pub fn tip(mut self, tip: impl Into<I256>) -> Self {
        self.tip = Some(tip.into());
        self
    }

    /// Set `max_base_fee`
    pub fn max_base_fee(mut self, max_base_fee: impl Into<U256>) -> Self {
        self.max_base_fee = Some(max_base_fee.into());
        self
    }

    /// Set `not_before`
    pub fn not_before(mut self, not_before: impl Into<U256>) -> Self {
        self.not_before = Some(not_before.into().low_u64().into());
        self
    }

    /// Set `deadline`
    pub fn deadline(mut self, deadline: impl Into<U256>) -> Self {
        self.deadline = Some(deadline.into().low_u64().into());
        self
    }

    /// Set `nonce`
    pub fn nonce(mut self, nonce: impl Into<U256>) -> Self {
        self.nonce = Some(nonce.into());
        self
    }

    /// Sign this [`MevTx`] with the specified signer. Converts into a
    /// [`SignedMevTxBuilder`], which has an `async fn build`
    pub fn with_signer<S: Signer>(self, signer: &S) -> SignedMevTxBuilderInternal<C, S> {
        SignedMevTxBuilderInternal {
            builder: self,
            signer,
        }
    }

    /// Set the chain_id explicitly. If this is not called, it will be signed
    /// with the signer's current chain ID.
    pub fn chain_id(mut self, chain_id: u64) -> Self {
        self.chain_id = Some(chain_id);
        self
    }

    /// Return the list of mandatory keys that are not yet set.
    pub fn missing_keys(&self) -> Vec<&'static str> {
        let mut v = vec![];
        if self.wallet.is_none() {
            v.push("wallet");
        }
        if self.to.is_none() {
            v.push("to");
        }
        if self.tip.is_none() {
            v.push("tip");
        }
        if self.nonce.is_none() {
            v.push("nonce");
        }
        v
    }
}

impl<M> MevTxBuilderInternal<MevWalletV1<M>>
where
    M: Middleware + 'static,
{
    /// Create a [`MevTxBuilder`] from an ethers `TypedTransaction`. This
    /// convenience method allows users to convert regular contract
    /// interactions into MEV txns easily.
    pub async fn from_typed_tx(
        contract: &MevWalletV1<M>,
        typed_tx: &ethers::types::transaction::eip2718::TypedTransaction,
    ) -> Result<Self, BuilderError>
    where
        M: Middleware,
    {
        let mut builder = MevTxBuilder::default().wallet(contract);

        match typed_tx.to() {
            Some(ethers::types::NameOrAddress::Address(addr)) => builder = builder.to(*addr),
            Some(ethers::types::NameOrAddress::Name(name)) => {
                let addr = contract
                    .client()
                    .resolve_name(name)
                    .await
                    .map_err(|e| BuilderError::Custom(format!("{}", e)))?;
                builder = builder.to(addr)
            }
            _ => {}
        }
        if let Some(data) = typed_tx.data() {
            builder = builder.data(data.clone());
        }
        if let Some(value) = typed_tx.value() {
            builder = builder.value(ethers::types::I256::from_raw(*value));
        }
        if let Some(t) = typed_tx.as_eip1559_ref() {
            if let (Some(max_fee), Some(max_priority)) =
                (t.max_fee_per_gas, t.max_priority_fee_per_gas)
            {
                builder = builder.max_base_fee(max_fee - max_priority);
            }
        }
        Ok(builder)
    }

    /// Populate the nonce by querying the contract. Note that multiple calls
    /// will return the same nonce until the chain confirms a transaction. If
    /// building multiple offline txns, consider [`populate_nonce_with_offset`]
    pub async fn populate_nonce(mut self) -> Result<Self, BuilderError>
    where
        M: Middleware + 'static,
    {
        let contract = self.wallet.as_ref().ok_or(BuilderError::MissingContract)?;
        if self.nonce.is_none() {
            self.nonce = Some(
                contract
                    .nonce()
                    .await
                    .map_err(|e| BuilderError::Custom(format!("{}", e)))?,
            );
        }
        Ok(self)
    }

    /// Populate the nonce by querying the contract, then add `offset` to the
    /// result. Useful for making sequences of transactions.
    pub async fn populate_nonce_with_offset(mut self, offset: u64) -> Result<Self, BuilderError>
    where
        M: Middleware + 'static,
    {
        self = self.populate_nonce().await?;
        self.nonce.map(|n| n + offset);
        Ok(self)
    }

    /// Populate the max base fee using the chain's EIP1559 estimator
    pub async fn populate_max_base_fee(mut self) -> Result<Self, BuilderError>
    where
        M: Middleware + 'static,
    {
        let provider = self
            .wallet
            .as_ref()
            .ok_or(BuilderError::MissingContract)?
            .client();

        if self.max_base_fee.is_none() {
            let (max_fee_per_gas, _) = provider
                .estimate_eip1559_fees(None)
                .await
                .map_err(|e| BuilderError::Custom(format!("{}", e)))?;
            self.max_base_fee = Some(max_fee_per_gas);
        }

        Ok(self)
    }

    /// Populate the transaction using the contract. Fills nonce and
    /// max_base_fee, if they are not already filled.
    pub async fn populate(self) -> Result<Self, BuilderError>
    where
        M: Middleware + 'static,
    {
        self.populate_nonce().await?.populate_max_base_fee().await
    }

    /// Build
    pub fn build(self) -> Result<MevTx, BuilderError> {
        let missing = self.missing_keys();
        if !missing.is_empty() {
            return Err(BuilderError::MissingKeys(missing));
        }

        let timing = self.not_before.unwrap_or_default() << 64 | self.deadline.unwrap_or_default();

        Ok(MevTx {
            chain_id: self.chain_id.unwrap_or(1),
            wallet: self.wallet.expect("checked by missing_keys").address(),
            to: self.to.expect("checked by missing_keys"),
            data: self.data.unwrap_or_default(),
            value: self.value.unwrap_or_default(),
            delegate: self.delegate.unwrap_or_default(),
            tip: self.tip.expect("checked by missing_keys"),
            max_base_fee: self.max_base_fee.unwrap_or_default(),
            timing,
            nonce: self.nonce.expect("checked by missing_keys"),
        })
    }
}

/// A [`SignedMevTxBuilder`] builds [`SignedMevTx`]. It is created from a
/// builder with a signer
#[derive(Debug, serde::Serialize)]

pub struct SignedMevTxBuilderInternal<'a, M, S> {
    builder: MevTxBuilderInternal<M>,
    signer: &'a S,
}

impl<'a, M, S> Clone for SignedMevTxBuilderInternal<'a, M, S>
where
    M: Clone,
{
    fn clone(&self) -> Self {
        Self {
            builder: self.builder.clone(),
            signer: self.signer,
        }
    }
}

delegate_builder!(SignedMevTxBuilderInternal<'a, M, S>);
delegate_builder_populate!(SignedMevTxBuilderInternal<'a, MevWalletV1<M>, S>);

impl<'a, M, S: Signer> SignedMevTxBuilderInternal<'a, M, S>
where
    S: Signer,
{
    /// Set the MevWallet from which this tx will be sent
    pub fn wallet<N>(
        self,
        contract: &MevWalletV1<N>,
    ) -> SignedMevTxBuilderInternal<'a, MevWalletV1<N>, S>
    where
        N: Middleware + 'static,
    {
        SignedMevTxBuilderInternal {
            builder: self.builder.wallet(contract),
            signer: self.signer,
        }
    }

    /// Replace the signer
    pub fn with_signer<T: Signer>(self, signer: &T) -> SignedMevTxBuilderInternal<M, T> {
        self.builder.with_signer(signer)
    }
}

impl<'a, M, S> SignedMevTxBuilderInternal<'a, MevWalletV1<M>, S>
where
    M: Middleware + 'static,
    S: Signer,
{
    /// Build and sign the transaction
    pub async fn build(self) -> Result<SignedMevTx, BuilderError> {
        let tx = self.builder.build()?;

        if self.signer.chain_id() != tx.chain_id {
            return Err(BuilderError::ChainIdMismatch {
                tx: tx.chain_id,
                signer: self.signer.chain_id(),
            });
        }

        tx.sign(self.signer)
            .await
            .map_err(|e| format!("{}", e))
            .map_err(BuilderError::Custom)
    }
}
