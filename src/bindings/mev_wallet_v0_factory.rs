pub use mev_wallet_v0_factory::*;
#[allow(clippy::too_many_arguments, non_camel_case_types)]
pub mod mev_wallet_v0_factory {
    #![allow(clippy::enum_variant_names)]
    #![allow(dead_code)]
    #![allow(clippy::type_complexity)]
    #![allow(unused_imports)]
    use ethers::contract::{
        builders::{ContractCall, Event},
        Contract, Lazy,
    };
    use ethers::core::{
        abi::{Abi, Detokenize, InvalidOutputType, Token, Tokenizable},
        types::*,
    };
    use ethers::providers::Middleware;
    #[doc = "MevWalletV0Factory was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    # [rustfmt :: skip] const __ABI : & str = "[{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_impl\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"constructor\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"salt\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"owner\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"createWallet\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"salt\",\"type\":\"bytes32\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"createWallet\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]}]" ;
    #[doc = r" The parsed JSON-ABI of the contract."]
    pub static MEVWALLETV0FACTORY_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            ethers::core::utils::__serde_json::from_str(__ABI).expect("invalid abi")
        });
    #[doc = r" Bytecode of the #name contract"]
    pub static MEVWALLETV0FACTORY_BYTECODE: ethers::contract::Lazy<ethers::core::types::Bytes> =
        ethers::contract::Lazy::new(|| {
            "0x60a03461007757601f61033f38819003918201601f19168301916001600160401b0383118484101761007c5780849260209460405283398101031261007757516001600160a01b03811681036100775760601b6001600160601b0319166080526040516102ac908161009382396080518161019f0152f35b600080fd5b634e487b7160e01b600052604160045260246000fdfe6080604052600436101561001257600080fd5b6000803560e01c80630f6304181461006757631d6476051461003357600080fd5b3461006457602036600319011261006457602061005233600435610180565b6040516001600160a01b039091168152f35b80fd5b503461006457604036600319011261006457602435906001600160a01b03908183168303610064575061009e602092600435610180565b60405191168152f35b156100ae57565b600080fd5b50634e487b7160e01b600052604160045260246000fd5b90601f8019910116810190811067ffffffffffffffff8211176100ec57604052565b6100f46100b3565b604052565b91906020808401936000905b600182106101135750505050565b83516001600160a01b0316815292820192600191909101908201610105565b3d1561017b573d9067ffffffffffffffff821161016e575b60405191610162601f8201601f1916602001846100ca565b82523d6000602084013e565b6101766100b3565b61014a565b606090565b604051733d602d80600a3d3981f3363d3d373d3d3d363d7360601b81527f000000000000000000000000000000000000000000000000000000000000000060148201526e5af43d82803e903d91602b57fd5bf360881b60288201526000919060379083f5916001600160a01b03808416156102725791809161026293604051916020830183811067ffffffffffffffff821117610265575b60405216815260405161024e81610240602082019463189acdbd60e31b8652602483016100f9565b03601f1981018352826100ca565b519082865af161025c610132565b506100a7565b90565b61026d6100b3565b610218565b8280fdfea26469706673582212207d7e99091e5cd78f21513c2292f0d2ec36f48cf7fa3ab3953b91bce652f2da6764736f6c63430008110033" . parse () . expect ("invalid bytecode")
        });
    pub struct MevWalletV0Factory<M>(ethers::contract::Contract<M>);
    impl<M> Clone for MevWalletV0Factory<M> {
        fn clone(&self) -> Self {
            MevWalletV0Factory(self.0.clone())
        }
    }
    impl<M> std::ops::Deref for MevWalletV0Factory<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> std::fmt::Debug for MevWalletV0Factory<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(MevWalletV0Factory))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ethers::providers::Middleware> MevWalletV0Factory<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(address.into(), MEVWALLETV0FACTORY_ABI.clone(), client)
                .into()
        }
        #[doc = r" Constructs the general purpose `Deployer` instance based on the provided constructor arguments and sends it."]
        #[doc = r" Returns a new instance of a deployer that returns an instance of this contract after sending the transaction"]
        #[doc = r""]
        #[doc = r" Notes:"]
        #[doc = r" 1. If there are no constructor arguments, you should pass `()` as the argument."]
        #[doc = r" 1. The default poll duration is 7 seconds."]
        #[doc = r" 1. The default number of confirmations is 1 block."]
        #[doc = r""]
        #[doc = r""]
        #[doc = r" # Example"]
        #[doc = r""]
        #[doc = r" Generate contract bindings with `abigen!` and deploy a new contract instance."]
        #[doc = r""]
        #[doc = r" *Note*: this requires a `bytecode` and `abi` object in the `greeter.json` artifact."]
        #[doc = r""]
        #[doc = r" ```ignore"]
        #[doc = r" # async fn deploy<M: ethers::providers::Middleware>(client: ::std::sync::Arc<M>) {"]
        #[doc = r#"     abigen!(Greeter,"../greeter.json");"#]
        #[doc = r""]
        #[doc = r#"    let greeter_contract = Greeter::deploy(client, "Hello world!".to_string()).unwrap().send().await.unwrap();"#]
        #[doc = r"    let msg = greeter_contract.greet().call().await.unwrap();"]
        #[doc = r" # }"]
        #[doc = r" ```"]
        pub fn deploy<T: ethers::core::abi::Tokenize>(
            client: ::std::sync::Arc<M>,
            constructor_args: T,
        ) -> ::std::result::Result<
            ethers::contract::builders::ContractDeployer<M, Self>,
            ethers::contract::ContractError<M>,
        > {
            let factory = ethers::contract::ContractFactory::new(
                MEVWALLETV0FACTORY_ABI.clone(),
                MEVWALLETV0FACTORY_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        #[doc = "Calls the contract's `createWallet` (0x0f630418) function"]
        pub fn create_wallet_with_owner(
            &self,
            salt: [u8; 32],
            owner: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([15, 99, 4, 24], (salt, owner))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `createWallet` (0x1d647605) function"]
        pub fn create_wallet(
            &self,
            salt: [u8; 32],
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([29, 100, 118, 5], salt)
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>>
        for MevWalletV0Factory<M>
    {
        fn from(contract: ethers::contract::Contract<M>) -> Self {
            Self(contract)
        }
    }
    #[doc = "Container type for all input parameters for the `createWallet` function with signature `createWallet(bytes32,address)` and selector `0x0f630418`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "createWallet", abi = "createWallet(bytes32,address)")]
    pub struct CreateWalletWithOwnerCall {
        pub salt: [u8; 32],
        pub owner: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `createWallet` function with signature `createWallet(bytes32)` and selector `0x1d647605`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "createWallet", abi = "createWallet(bytes32)")]
    pub struct CreateWalletCall {
        pub salt: [u8; 32],
    }
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum MevWalletV0FactoryCalls {
        CreateWalletWithOwner(CreateWalletWithOwnerCall),
        CreateWallet(CreateWalletCall),
    }
    impl ethers::core::abi::AbiDecode for MevWalletV0FactoryCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::std::result::Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <CreateWalletWithOwnerCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MevWalletV0FactoryCalls::CreateWalletWithOwner(decoded));
            }
            if let Ok(decoded) =
                <CreateWalletCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MevWalletV0FactoryCalls::CreateWallet(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for MevWalletV0FactoryCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                MevWalletV0FactoryCalls::CreateWalletWithOwner(element) => element.encode(),
                MevWalletV0FactoryCalls::CreateWallet(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for MevWalletV0FactoryCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                MevWalletV0FactoryCalls::CreateWalletWithOwner(element) => element.fmt(f),
                MevWalletV0FactoryCalls::CreateWallet(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<CreateWalletWithOwnerCall> for MevWalletV0FactoryCalls {
        fn from(var: CreateWalletWithOwnerCall) -> Self {
            MevWalletV0FactoryCalls::CreateWalletWithOwner(var)
        }
    }
    impl ::std::convert::From<CreateWalletCall> for MevWalletV0FactoryCalls {
        fn from(var: CreateWalletCall) -> Self {
            MevWalletV0FactoryCalls::CreateWallet(var)
        }
    }
    #[doc = "Container type for all return fields from the `createWallet` function with signature `createWallet(bytes32,address)` and selector `0x0f630418`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct CreateWalletWithOwnerReturn(pub ethers::core::types::Address);
    #[doc = "Container type for all return fields from the `createWallet` function with signature `createWallet(bytes32)` and selector `0x1d647605`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct CreateWalletReturn(pub ethers::core::types::Address);
}
