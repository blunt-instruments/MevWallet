use ethers::{
    prelude::{Address, Bytes, EthCall, U256},
    providers::Middleware,
    signers::Signer,
    types::{transaction::eip2718::TypedTransaction, Eip1559TransactionRequest, I256},
};

use crate::{MevTx, MevWalletV0, SignedMevTx};

/// Error type for [`MevTxBuilder`]
#[derive(Debug, thiserror::Error)]
pub enum BuilderError {
    /// Missing keys
    #[error("Missing Keys: {0:?}")]
    MissingKeys(Vec<&'static str>),
    /// Custom error (e.g. signer)
    #[error("(0)")]
    Custom(String),
}

/// A Builder for `MevTx`
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct MevTxBuilder {
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

impl From<&MevTxBuilder> for TypedTransaction {
    fn from(val: &MevTxBuilder) -> Self {
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

impl MevTxBuilder {
    /// Instantiate an empty builder
    pub fn new() -> Self {
        Default::default()
    }

    /// Create a [`MevTxBuilder`] from an ethers `TypedTransaction`. This
    /// convenience method allows users to convert regular contract
    /// interactions into MEV txns easily.
    pub async fn from_typed_tx<M>(
        client: &M,
        typed_tx: &ethers::types::transaction::eip2718::TypedTransaction,
    ) -> Result<MevTxBuilder, BuilderError>
    where
        M: Middleware,
    {
        let mut builder = MevTxBuilder::default();
        match typed_tx.to() {
            Some(ethers::types::NameOrAddress::Address(addr)) => builder = builder.to(*addr),
            Some(ethers::types::NameOrAddress::Name(name)) => {
                let addr = client
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
    pub fn with_signer<S: Signer>(self, signer: S) -> SignedMevTxBuilder<S> {
        SignedMevTxBuilder {
            builder: self,
            signer,
        }
    }

    /// Return the list of mandatory keys that are not yet set.
    pub fn missing_keys(&self) -> Vec<&'static str> {
        let mut v = vec![];
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

    /// Populate the nonce by querying the contract. Note that multiple calls
    /// will return the same nonce until the chain confirms a transaction. If
    /// building multiple offline txns, consider [`populate_nonce_with_offset`]
    pub async fn populate_nonce<M>(
        mut self,
        contract: &MevWalletV0<M>,
    ) -> Result<Self, BuilderError>
    where
        M: Middleware + 'static,
    {
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
    pub async fn populate_nonce_with_offset<M>(
        mut self,
        contract: &MevWalletV0<M>,
        offset: u64,
    ) -> Result<Self, BuilderError>
    where
        M: Middleware + 'static,
    {
        self = self.populate_nonce(contract).await?;
        self.nonce.map(|n| n + offset);
        Ok(self)
    }

    /// Populate the max base fee using the chain's EIP1559 estimator
    pub async fn populate_max_base_fee<M>(
        mut self,
        contract: &MevWalletV0<M>,
    ) -> Result<Self, BuilderError>
    where
        M: Middleware + 'static,
    {
        let provider = contract.client();

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
    pub async fn populate<M>(self, contract: &MevWalletV0<M>) -> Result<Self, BuilderError>
    where
        M: Middleware + 'static,
    {
        self.populate_nonce(contract)
            .await?
            .populate_max_base_fee(contract)
            .await
    }

    /// Build
    pub fn build(self) -> Result<MevTx, BuilderError> {
        let missing = self.missing_keys();
        if !missing.is_empty() {
            return Err(BuilderError::MissingKeys(missing));
        }

        let timing = self.not_before.unwrap_or_default() << 64 | self.deadline.unwrap_or_default();

        Ok(MevTx {
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
#[derive(Debug, Clone, serde::Serialize)]
pub struct SignedMevTxBuilder<S> {
    #[serde(flatten)]
    builder: MevTxBuilder,
    #[serde(skip)]
    signer: S,
}

impl<S: Signer> SignedMevTxBuilder<S> {
    /// Set `to`
    pub fn to(mut self, to: Address) -> Self {
        self.builder = self.builder.to(to);
        self
    }

    /// Set `data`
    pub fn data(mut self, data: impl Into<Bytes>) -> Self {
        self.builder = self.builder.data(data);
        self
    }

    /// Set `value`
    pub fn value(mut self, value: impl Into<I256>) -> Self {
        self.builder = self.builder.value(value);
        self
    }

    /// Set `delegate`
    pub fn delegate(mut self, delegate: bool) -> Self {
        self.builder = self.builder.delegate(delegate);
        self
    }

    /// Set `tip`
    pub fn tip(mut self, tip: impl Into<I256>) -> Self {
        self.builder = self.builder.tip(tip);
        self
    }

    /// Set `max_base_fee`
    pub fn max_base_fee(mut self, max_base_fee: impl Into<U256>) -> Self {
        self.builder = self.builder.max_base_fee(max_base_fee);
        self
    }

    /// Set `not_before`
    pub fn not_before(mut self, not_before: impl Into<U256>) -> Self {
        self.builder = self.builder.not_before(not_before);
        self
    }

    /// Set `deadline`
    pub fn deadline(mut self, deadline: impl Into<U256>) -> Self {
        self.builder = self.builder.deadline(deadline);
        self
    }

    /// Set `nonce`
    pub fn nonce(mut self, nonce: impl Into<U256>) -> Self {
        self.builder = self.builder.nonce(nonce);
        self
    }

    /// Replace the signer
    pub fn with_signer<T: Signer>(self, signer: T) -> SignedMevTxBuilder<T> {
        let signer = signer.with_chain_id(self.signer.chain_id());
        self.builder.with_signer(signer)
    }

    /// Return the list of mandatory keys that are not yet set.
    pub fn missing_keys(&self) -> Vec<&'static str> {
        self.builder.missing_keys()
    }

    /// Populate the nonce by querying the contract. Note that multiple calls
    /// will return the same nonce until the chain confirms a transaction. If
    /// building multiple offline txns, consider [`populate_nonce_with_offset`]
    pub async fn populate_nonce<M>(
        mut self,
        contract: &MevWalletV0<M>,
    ) -> Result<Self, BuilderError>
    where
        M: Middleware + 'static,
    {
        self.builder = self.builder.populate_nonce(contract).await?;
        Ok(self)
    }

    /// Populate the nonce by querying the contract, then add `offset` to the
    /// result. Useful for making sequences of transactions.
    pub async fn populate_nonce_with_offset<M>(
        mut self,
        contract: &MevWalletV0<M>,
        offset: u64,
    ) -> Result<Self, BuilderError>
    where
        M: Middleware + 'static,
    {
        self.builder = self
            .builder
            .populate_nonce_with_offset(contract, offset)
            .await?;
        Ok(self)
    }

    /// Populate the max base fee using the chain's EIP1559 estimator
    pub async fn populate_max_base_fee<M>(
        mut self,
        contract: &MevWalletV0<M>,
    ) -> Result<Self, BuilderError>
    where
        M: Middleware + 'static,
    {
        self.builder = self.builder.populate_max_base_fee(contract).await?;
        Ok(self)
    }

    /// Populate the transaction using the contract. Fills gas, nonce, and
    /// max_base_fee, if they are not already filled.
    pub async fn populate<M>(mut self, contract: &MevWalletV0<M>) -> Result<Self, BuilderError>
    where
        M: Middleware + 'static,
    {
        self.builder = self.builder.populate(contract).await?;
        Ok(self)
    }

    /// Set the chain_id explicitly. If this is not called, it will be signed
    /// with the signer's current chain ID.
    pub fn chain_id(mut self, chain_id: u64) -> Self {
        self.signer = self.signer.with_chain_id(chain_id);
        self
    }

    /// Build and sign the transaction
    pub async fn build(self, contract: Address) -> Result<SignedMevTx, BuilderError> {
        let tx = self.builder.build()?;

        tx.sign(contract, &self.signer)
            .await
            .map_err(|e| format!("{}", e))
            .map_err(BuilderError::Custom)
    }
}
