/// MevTx Builders
pub mod builder;

/// MevTx structs
pub mod types;

pub use builder::*;
pub use types::*;

use crate::{
    bindings::mev_wallet_v0_factory::MevWalletV0Factory, MevWalletV0, MEV_WALLET_PROXY_FACTORY_ADDR,
};
use ethers::{
    prelude::{builders::ContractCall, ContractError},
    providers::{Middleware, ProviderError},
    types::Address,
};
use std::sync::Arc;

/// Create a new proxy with a specific salt
pub fn deploy_proxy<M>(client: Arc<M>, salt: impl Into<[u8; 32]>) -> ContractCall<M, Address>
where
    M: Middleware,
{
    let factory = MevWalletV0Factory::new(*MEV_WALLET_PROXY_FACTORY_ADDR, client);

    factory.create_wallet(salt.into())
}

/// Create a new proxy with a specific salt and owner
pub fn deploy_proxy_with_owner<M>(
    client: Arc<M>,
    salt: impl Into<[u8; 32]>,
    owner: impl Into<Address>,
) -> ContractCall<M, Address>
where
    M: Middleware,
{
    let factory = MevWalletV0Factory::new(*MEV_WALLET_PROXY_FACTORY_ADDR, client);

    factory.create_wallet_with_owner(salt.into(), owner.into())
}

impl<M: ethers::providers::Middleware> crate::MevWalletV0<M> {
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
            .map_err(ContractError::MiddlewareError)?;
        if receipt.is_none() {
            return Err(ContractError::MiddlewareError(M::convert_err(
                ProviderError::CustomError("Could not get deploy tx receipt".to_owned()),
            )));
        }
        Ok(MevWalletV0::new(address, client))
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
            .map_err(ContractError::MiddlewareError)?;
        if receipt.is_none() {
            return Err(ContractError::MiddlewareError(M::convert_err(
                ProviderError::CustomError("Could not get deploy tx receipt".to_owned()),
            )));
        }
        Ok(MevWalletV0::new(address, client))
    }

    /// Create a [`MevTxBuilder`] from an ethers `TypedTransaction`. This
    /// convenience method allows users to convert regular contract
    /// interactions into MEV txns easily.
    pub async fn convert_tx(
        &self,
        typed_tx: &ethers::types::transaction::eip2718::TypedTransaction,
    ) -> Result<MevTxBuilder, BuilderError> {
        MevTxBuilder::from_typed_tx(&self.client(), typed_tx).await
    }

    /// Create a contract call object sending the signed mev transaction
    pub fn send(&self, tx: SignedMevTx) -> ethers::prelude::builders::ContractCall<M, ()> {
        tx.into_call(self.client())
    }
}
