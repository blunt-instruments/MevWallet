pub use mev_wallet_v1_factory::*;
#[allow(clippy::too_many_arguments, non_camel_case_types)]
pub mod mev_wallet_v1_factory {
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
    #[doc = "MevWalletV1Factory was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    # [rustfmt :: skip] const __ABI : & str = "[{\"inputs\":[],\"type\":\"error\",\"name\":\"CreationFailed\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"bytes\",\"name\":\"\",\"type\":\"bytes\",\"components\":[]}],\"type\":\"error\",\"name\":\"InitFailed\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"Proxy\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"salt\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"owner\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"createWallet\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"salt\",\"type\":\"bytes32\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"createWallet\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]}]" ;
    #[doc = r" The parsed JSON-ABI of the contract."]
    pub static MEVWALLETV1FACTORY_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> =
        ::ethers::contract::Lazy::new(|| {
            ::ethers::core::utils::__serde_json::from_str(__ABI).expect("invalid abi")
        });
    #[doc = r" Bytecode of the #name contract"]
    pub static MEVWALLETV1FACTORY_BYTECODE: ::ethers::contract::Lazy<::ethers::core::types::Bytes> =
        ::ethers::contract::Lazy::new(|| {
            "0x60808060405234610016576104b2908161001c8239f35b600080fdfe6080604052600436101561001257600080fd5b6000803560e01c80630f6304181461009157631d6476051461003357600080fd5b3461008e5760207ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc36011261008e576020610070336004356102b1565b73ffffffffffffffffffffffffffffffffffffffff60405191168152f35b80fd5b503461008e5760407ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc36011261008e576024359073ffffffffffffffffffffffffffffffffffffffff90818316830361008e57506100f36020926004356102b1565b60405191168152f35b507f4e487b7100000000000000000000000000000000000000000000000000000000600052604160045260246000fd5b90601f7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0910116810190811067ffffffffffffffff82111761016d57604052565b6101756100fc565b604052565b604051906020820182811067ffffffffffffffff82111761016d57604052565b91906020808401936000905b600182106101b45750505050565b828060019273ffffffffffffffffffffffffffffffffffffffff8751168152019401910190926101a6565b3d15610246573d9067ffffffffffffffff8211610239575b6040519161022d60207fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0601f840116018461012c565b82523d6000602084013e565b6102416100fc565b6101f7565b606090565b60208082528251818301819052939260005b85811061029d575050507fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0601f8460006040809697860101520116010190565b81810183015184820160400152820161025d565b604080516020810192835273ffffffffffffffffffffffffffffffffffffffff8416818301528181527fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe09391929061030a60608261012c565b519020603283517f3d602880600a3d3981f3363d3d373d3d3d363d6ec08718718b974d644b098c1981527fbd00645af43d82803e903d91602657fd5bf3000000000000000000000000000060208201526000f59273ffffffffffffffffffffffffffffffffffffffff84161561047c5760009182916103a661038a61017a565b73ffffffffffffffffffffffffffffffffffffffff9093168352565b6103ed855191826103e160208201957fc4d66de80000000000000000000000000000000000000000000000000000000087526024830161019a565b0390810183528261012c565b519082865af16103fb6101df565b901561044457505173ffffffffffffffffffffffffffffffffffffffff821681527e70c3b37cbd33a8f91033a749a5e534a86edc8c2a4c6155b9c53e99ffd5943790602090a190565b61047891519182917f225d0a580000000000000000000000000000000000000000000000000000000083526004830161024b565b0390fd5b600483517fd786d393000000000000000000000000000000000000000000000000000000008152fdfea164736f6c6343000811000a" . parse () . expect ("invalid bytecode")
        });
    pub struct MevWalletV1Factory<M>(::ethers::contract::Contract<M>);
    impl<M> Clone for MevWalletV1Factory<M> {
        fn clone(&self) -> Self {
            MevWalletV1Factory(self.0.clone())
        }
    }
    impl<M> std::ops::Deref for MevWalletV1Factory<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> std::fmt::Debug for MevWalletV1Factory<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(MevWalletV1Factory))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> MevWalletV1Factory<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ::ethers::contract::Contract::new(
                address.into(),
                MEVWALLETV1FACTORY_ABI.clone(),
                client,
            )
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
        pub fn deploy<T: ::ethers::core::abi::Tokenize>(
            client: ::std::sync::Arc<M>,
            constructor_args: T,
        ) -> ::std::result::Result<
            ::ethers::contract::builders::ContractDeployer<M, Self>,
            ::ethers::contract::ContractError<M>,
        > {
            let factory = ::ethers::contract::ContractFactory::new(
                MEVWALLETV1FACTORY_ABI.clone(),
                MEVWALLETV1FACTORY_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        #[doc = "Calls the contract's `createWallet` (0x0f630418) function"]
        pub fn create_wallet_with_owner(
            &self,
            salt: [u8; 32],
            owner: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([15, 99, 4, 24], (salt, owner))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `createWallet` (0x1d647605) function"]
        pub fn create_wallet(
            &self,
            salt: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([29, 100, 118, 5], salt)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Gets the contract's `Proxy` event"]
        pub fn proxy_filter(&self) -> ::ethers::contract::builders::Event<M, ProxyFilter> {
            self.0.event()
        }
        #[doc = r" Returns an [`Event`](#ethers_contract::builders::Event) builder for all events of this contract"]
        pub fn events(&self) -> ::ethers::contract::builders::Event<M, ProxyFilter> {
            self.0.event_with_filter(Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
        for MevWalletV1Factory<M>
    {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self(contract)
        }
    }
    #[doc = "Custom Error type `CreationFailed` with signature `CreationFailed()` and selector `0xd786d393`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        :: ethers :: contract :: EthError,
        :: ethers :: contract :: EthDisplay,
    )]
    #[etherror(name = "CreationFailed", abi = "CreationFailed()")]
    pub struct CreationFailed;
    #[doc = "Custom Error type `InitFailed` with signature `InitFailed(bytes)` and selector `0x225d0a58`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        :: ethers :: contract :: EthError,
        :: ethers :: contract :: EthDisplay,
    )]
    #[etherror(name = "InitFailed", abi = "InitFailed(bytes)")]
    pub struct InitFailed(pub ::ethers::core::types::Bytes);
    #[derive(Debug, Clone, PartialEq, Eq, :: ethers :: contract :: EthAbiType)]
    pub enum MevWalletV1FactoryErrors {
        CreationFailed(CreationFailed),
        InitFailed(InitFailed),
    }
    impl ::ethers::core::abi::AbiDecode for MevWalletV1FactoryErrors {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::std::result::Result<Self, ::ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <CreationFailed as ::ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MevWalletV1FactoryErrors::CreationFailed(decoded));
            }
            if let Ok(decoded) =
                <InitFailed as ::ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MevWalletV1FactoryErrors::InitFailed(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for MevWalletV1FactoryErrors {
        fn encode(self) -> Vec<u8> {
            match self {
                MevWalletV1FactoryErrors::CreationFailed(element) => element.encode(),
                MevWalletV1FactoryErrors::InitFailed(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for MevWalletV1FactoryErrors {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                MevWalletV1FactoryErrors::CreationFailed(element) => element.fmt(f),
                MevWalletV1FactoryErrors::InitFailed(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<CreationFailed> for MevWalletV1FactoryErrors {
        fn from(var: CreationFailed) -> Self {
            MevWalletV1FactoryErrors::CreationFailed(var)
        }
    }
    impl ::std::convert::From<InitFailed> for MevWalletV1FactoryErrors {
        fn from(var: InitFailed) -> Self {
            MevWalletV1FactoryErrors::InitFailed(var)
        }
    }
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        :: ethers :: contract :: EthEvent,
        :: ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethevent(name = "Proxy", abi = "Proxy(address)")]
    pub struct ProxyFilter(pub ::ethers::core::types::Address);
    #[doc = "Container type for all input parameters for the `createWallet` function with signature `createWallet(bytes32,address)` and selector `0x0f630418`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        :: ethers :: contract :: EthCall,
        :: ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "createWallet", abi = "createWallet(bytes32,address)")]
    pub struct CreateWalletWithOwnerCall {
        pub salt: [u8; 32],
        pub owner: ::ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `createWallet` function with signature `createWallet(bytes32)` and selector `0x1d647605`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        :: ethers :: contract :: EthCall,
        :: ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "createWallet", abi = "createWallet(bytes32)")]
    pub struct CreateWalletCall {
        pub salt: [u8; 32],
    }
    #[derive(Debug, Clone, PartialEq, Eq, :: ethers :: contract :: EthAbiType)]
    pub enum MevWalletV1FactoryCalls {
        CreateWalletWithOwner(CreateWalletWithOwnerCall),
        CreateWallet(CreateWalletCall),
    }
    impl ::ethers::core::abi::AbiDecode for MevWalletV1FactoryCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::std::result::Result<Self, ::ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <CreateWalletWithOwnerCall as ::ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MevWalletV1FactoryCalls::CreateWalletWithOwner(decoded));
            }
            if let Ok(decoded) =
                <CreateWalletCall as ::ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MevWalletV1FactoryCalls::CreateWallet(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for MevWalletV1FactoryCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                MevWalletV1FactoryCalls::CreateWalletWithOwner(element) => element.encode(),
                MevWalletV1FactoryCalls::CreateWallet(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for MevWalletV1FactoryCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                MevWalletV1FactoryCalls::CreateWalletWithOwner(element) => element.fmt(f),
                MevWalletV1FactoryCalls::CreateWallet(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<CreateWalletWithOwnerCall> for MevWalletV1FactoryCalls {
        fn from(var: CreateWalletWithOwnerCall) -> Self {
            MevWalletV1FactoryCalls::CreateWalletWithOwner(var)
        }
    }
    impl ::std::convert::From<CreateWalletCall> for MevWalletV1FactoryCalls {
        fn from(var: CreateWalletCall) -> Self {
            MevWalletV1FactoryCalls::CreateWallet(var)
        }
    }
    #[doc = "Container type for all return fields from the `createWallet` function with signature `createWallet(bytes32,address)` and selector `0x0f630418`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        :: ethers :: contract :: EthAbiType,
        :: ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct CreateWalletWithOwnerReturn(pub ::ethers::core::types::Address);
    #[doc = "Container type for all return fields from the `createWallet` function with signature `createWallet(bytes32)` and selector `0x1d647605`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        :: ethers :: contract :: EthAbiType,
        :: ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct CreateWalletReturn(pub ::ethers::core::types::Address);
}
