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

/// crate internal macros
mod macros;

/// Tooling for deploying new MevWallet proxies
pub mod deploy;

use ethers::types::H160;

/// Mev transaction tx typehash:
/// `fd4c9c9ceea85482c3671626f7b1c65bc2230325b86dbcb0f971327c3062c26a`
///
/// `keccak256(
///         "MevTx(address to,bytes data,int256 value,bool delegate,int256 tip,uint256 maxBaseFee,uint256 timing,uint256 nonce)"
///     )`
pub const TX_TYPEHASH: [u8; 32] = [
    253, 76, 156, 156, 238, 168, 84, 130, 195, 103, 22, 38, 247, 177, 198, 91, 194, 35, 3, 37, 184,
    109, 188, 176, 249, 113, 50, 124, 48, 98, 194, 106,
];

/// MevWeth address: `0x00000000008C43efC014746c230049e330039Cb3`
pub const MEV_WETH_ADDR: H160 = H160([
    0, 0, 0, 0, 0, 140, 67, 239, 192, 20, 116, 108, 35, 0, 73, 227, 48, 3, 156, 179,
]);

/// MevWalletProxyFactory address: `0x9248B5e672e1880af34068C0FaE18D30c26D05Fb`
pub const MEV_WALLET_PROXY_FACTORY_ADDR: H160 = H160([
    146, 72, 181, 230, 114, 225, 136, 10, 243, 64, 104, 192, 250, 225, 141, 48, 194, 109, 5, 251,
]);

#[cfg(test)]
mod test {
    use super::*;
    use ethers::utils::hex;

    #[test]
    fn test_const_wallets() {
        assert_eq!(
            MEV_WALLET_PROXY_FACTORY_ADDR,
            "0x9248B5e672e1880af34068C0FaE18D30c26D05Fb"
                .parse()
                .unwrap()
        );
        assert_eq!(
            MEV_WETH_ADDR,
            "0x00000000008C43efC014746c230049e330039Cb3"
                .parse()
                .unwrap()
        );
    }

    #[test]
    fn it_calculates_the_typehash() {
        assert_eq!(
            TX_TYPEHASH,
            ethers::utils::keccak256(
        "MevTx(address to,bytes data,int256 value,bool delegate,int256 tip,uint256 maxBaseFee,uint256 timing,uint256 nonce)".as_bytes(),
    )
        );
        assert_eq!(
            TX_TYPEHASH.to_vec(),
            hex::decode("fd4c9c9ceea85482c3671626f7b1c65bc2230325b86dbcb0f971327c3062c26a")
                .unwrap()
        );
    }
}
