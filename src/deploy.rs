use ethers::{prelude::builders::ContractCall, providers::Middleware, types::Address};
use std::sync::Arc;

use crate::{bindings::mev_wallet_v0_factory::MevWalletV0Factory, MEV_WALLET_PROXY_FACTORY_ADDR};

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
