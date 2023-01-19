#![warn(missing_docs, unreachable_pub)]
#![deny(unused_must_use)]
//! SDK for working with MEV-driven Meta-txns

/// Contract Bindings
#[allow(missing_docs)]
pub mod bindings;
pub use bindings::mev_wallet_v1::MevWalletV1;

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
    "0x49496DD21760ED9235aFE43871983869CC0eFC61"
        .parse()
        .unwrap()
});

#[cfg(test)]
mod test {
    use ethers::utils::hex;

    use crate::TX_TYPEHASH;

    #[test]
    fn it_calculates_the_typehash() {
        assert_eq!(
            *TX_TYPEHASH.to_vec(),
            hex::decode("fd4c9c9ceea85482c3671626f7b1c65bc2230325b86dbcb0f971327c3062c26a")
                .unwrap()
        );
    }
}
