/// Delegates the common builder calls to an internal builder named `builder`
#[macro_export]
macro_rules! delegate_builder {
    (
        $name:ident $(<
            // match one or more lifetimes separated by a comma
            $( $lt:lifetime ),*
            $( $generic:tt ),+
        >)?
    ) => {
        impl $(< $($lt),* $($generic),+ >)? $name $(< $($lt),* $($generic),+ >)? {

            /// Return the list of mandatory keys that are not yet set.
            pub fn missing_keys(&self) -> Vec<&'static str> {
                self.builder.missing_keys()
            }


            /// Set the chain_id explicitly. If this is not called, it will be signed
            /// with the signer's current chain ID.
            pub fn chain_id(mut self, chain_id: u64) -> Self {
                self.builder = self.builder.chain_id(chain_id);
                self
            }

            /// Set `to`
            pub fn to(mut self, to: ethers::types::Address) -> Self {
                self.builder = self.builder.to(to);
                self
            }

            /// Set `data`
            pub fn data(mut self, data: impl Into<::ethers::types::Bytes>) -> Self {
                self.builder = self.builder.data(data);
                self
            }

            /// Set `value`
            pub fn value(mut self, value: impl Into<::ethers::types::I256>) -> Self {
                self.builder = self.builder.value(value);
                self
            }

            /// Set `delegate`
            pub fn delegate(mut self, delegate: bool) -> Self {
                self.builder = self.builder.delegate(delegate);
                self
            }

            /// Set `tip`
            pub fn tip(mut self, tip: impl Into<::ethers::types::I256>) -> Self {
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
        }

    }
}

/// Implements async populate calls on builders that accept a builder with a
/// wallet
#[macro_export]
macro_rules! delegate_builder_populate {
    (
        $name:ident $(<
            // match one or more lifetimes separated by a comma
            $( $lt:lifetime ),*
            MevWalletV1<M>,
            $( $generic:tt ),*
        >)?
    ) => {
        impl $(< $($lt),* M, $($generic),+ >)? $name $(< $($lt),* $crate::MevWalletV1<M>, $($generic),+ >)?
        where
            M: Middleware + 'static
        {
            /// Populate the nonce by querying the contract. Note that multiple calls
            /// will return the same nonce until the chain confirms a transaction. If
            /// building multiple offline txns, consider [`populate_nonce_with_offset`]
            pub async fn populate_nonce(mut self) -> Result<Self, BuilderError>
            where
                M: Middleware + 'static,
            {
                self.builder = self.builder.populate_nonce().await?;
                Ok(self)
            }

            /// Populate the nonce by querying the contract, then add `offset` to the
            /// result. Useful for making sequences of transactions.
            pub async fn populate_nonce_with_offset(mut self, offset: u64) -> Result<Self, BuilderError>
            where
                M: Middleware + 'static,
            {
                self.builder = self.builder.populate_nonce_with_offset(offset).await?;
                Ok(self)
            }

            /// Populate the max base fee using the chain's EIP1559 estimator
            pub async fn populate_max_base_fee(mut self) -> Result<Self, BuilderError>
            where
                M: Middleware + 'static,
            {
                self.builder = self.builder.populate_max_base_fee().await?;
                Ok(self)
            }

            /// Populate the transaction using the contract. Fills gas, nonce, and
            /// max_base_fee, if they are not already filled.
            pub async fn populate(mut self) -> Result<Self, BuilderError>
            where
                M: Middleware + 'static,
            {
                self.builder = self.builder.populate().await?;
                Ok(self)
            }
        }
    }
}
