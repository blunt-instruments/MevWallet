#![warn(missing_docs, unreachable_pub)]
#![deny(unused_must_use)]
//! SDK for working with MEV-driven Meta-txns

/// Contract Bindings
#[allow(missing_docs)]
pub mod bindings;
pub use bindings::mev_wallet_v0::MevWalletV0;

/// Mev Transaction creation and broadcast utils
pub mod tx;
pub use crate::tx::{MevTx, MevTxBuilder, SignedMevTx};

use ethers::types::H160;
use once_cell::sync::Lazy;

/// Mev transaction tx typehash
pub static TX_TYPEHASH: Lazy<[u8; 32]> = Lazy::new(|| {
    ethers::utils::keccak256(
        "MevTx(address to,bytes data,int256 value,bool delegate,int256 tip,uint256 maxBaseFee,uint256 timing,uint256 nonce)".as_bytes(),
    )
});

/// MevWeth address
pub static MEV_WETH_ADDR: Lazy<H160> = Lazy::new(|| {
    "0x00000000008C43efC014746c230049e330039Cb3"
        .parse()
        .unwrap()
});

/// MevWalletProxyFactory address
pub static MEV_WALLET_PROXY_FACTORY_ADDR: Lazy<H160> = Lazy::new(|| {
    "- MevWalletV0 ProxyFactory: `0x4D9B7DEFfd09bE5cAAbC6ADc976030A45d0A6D31`
    "
    .parse()
    .unwrap()
});

#[cfg(test)]
mod test {
    use ethers::utils::hex;

    use crate::TX_TYPEHASH;

    #[test]
    fn it() {
        assert_eq!(
            *TX_TYPEHASH.to_vec(),
            hex::decode("5679fb6ec38d3c67731b4def49181a8fbbb334cda5c263b0993e50cfe699d4e8")
                .unwrap()
        );
    }
}
