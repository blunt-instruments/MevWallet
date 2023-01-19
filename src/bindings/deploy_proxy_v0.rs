pub use deploy_proxy_v0::*;
#[allow(clippy::too_many_arguments, non_camel_case_types)]
pub mod deploy_proxy_v0 {
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
    #[doc = "DeployProxyV0 was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    # [rustfmt :: skip] const __ABI : & str = "[{\"inputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"proxy\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"IS_SCRIPT\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\",\"components\":[]}],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"run\",\"outputs\":[]}]" ;
    #[doc = r" The parsed JSON-ABI of the contract."]
    pub static DEPLOYPROXYV0_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> =
        ::ethers::contract::Lazy::new(|| {
            ::ethers::core::utils::__serde_json::from_str(__ABI).expect("invalid abi")
        });
    #[doc = r" Bytecode of the #name contract"]
    pub static DEPLOYPROXYV0_BYTECODE: ::ethers::contract::Lazy<::ethers::core::types::Bytes> =
        ::ethers::contract::Lazy::new(|| {
            "0x6080806040523461003d57600c80546001600160a81b03191674444544a54b5193ba6d1a3cf9c83ee12422b6a8240117905561011890816100438239f35b600080fdfe608080604052600436101561001357600080fd5b600090813560e01c908163ef6ac0f01461007a575063f8ccbf471461003757600080fd5b3461007757807ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc36011261007757602060ff600c54166040519015158152f35b80fd5b9050346101075760207ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc36011261010757807f08c379a0000000000000000000000000000000000000000000000000000000006064925260206004820152601f60248201527f4d657657616c6c6574563020757365206e6f74207265636f6d6d656e646564006044820152fd5b5080fdfea164736f6c6343000811000a" . parse () . expect ("invalid bytecode")
        });
    pub struct DeployProxyV0<M>(::ethers::contract::Contract<M>);
    impl<M> Clone for DeployProxyV0<M> {
        fn clone(&self) -> Self {
            DeployProxyV0(self.0.clone())
        }
    }
    impl<M> std::ops::Deref for DeployProxyV0<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> std::fmt::Debug for DeployProxyV0<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(DeployProxyV0))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> DeployProxyV0<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ::ethers::contract::Contract::new(address.into(), DEPLOYPROXYV0_ABI.clone(), client)
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
                DEPLOYPROXYV0_ABI.clone(),
                DEPLOYPROXYV0_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        #[doc = "Calls the contract's `IS_SCRIPT` (0xf8ccbf47) function"]
        pub fn is_script(&self) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([248, 204, 191, 71], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `run` (0xef6ac0f0) function"]
        pub fn run(&self, p0: [u8; 32]) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([239, 106, 192, 240], p0)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Gets the contract's `proxy` event"]
        pub fn proxy_filter(&self) -> ::ethers::contract::builders::Event<M, ProxyFilter> {
            self.0.event()
        }
        #[doc = r" Returns an [`Event`](#ethers_contract::builders::Event) builder for all events of this contract"]
        pub fn events(&self) -> ::ethers::contract::builders::Event<M, ProxyFilter> {
            self.0.event_with_filter(Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
        for DeployProxyV0<M>
    {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self(contract)
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
    #[ethevent(name = "proxy", abi = "proxy(address)")]
    pub struct ProxyFilter(pub ::ethers::core::types::Address);
    #[doc = "Container type for all input parameters for the `IS_SCRIPT` function with signature `IS_SCRIPT()` and selector `0xf8ccbf47`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        :: ethers :: contract :: EthCall,
        :: ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "IS_SCRIPT", abi = "IS_SCRIPT()")]
    pub struct IsScriptCall;
    #[doc = "Container type for all input parameters for the `run` function with signature `run(bytes32)` and selector `0xef6ac0f0`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        :: ethers :: contract :: EthCall,
        :: ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "run", abi = "run(bytes32)")]
    pub struct RunCall(pub [u8; 32]);
    #[derive(Debug, Clone, PartialEq, Eq, :: ethers :: contract :: EthAbiType)]
    pub enum DeployProxyV0Calls {
        IsScript(IsScriptCall),
        Run(RunCall),
    }
    impl ::ethers::core::abi::AbiDecode for DeployProxyV0Calls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::std::result::Result<Self, ::ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <IsScriptCall as ::ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(DeployProxyV0Calls::IsScript(decoded));
            }
            if let Ok(decoded) = <RunCall as ::ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(DeployProxyV0Calls::Run(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for DeployProxyV0Calls {
        fn encode(self) -> Vec<u8> {
            match self {
                DeployProxyV0Calls::IsScript(element) => element.encode(),
                DeployProxyV0Calls::Run(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for DeployProxyV0Calls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                DeployProxyV0Calls::IsScript(element) => element.fmt(f),
                DeployProxyV0Calls::Run(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<IsScriptCall> for DeployProxyV0Calls {
        fn from(var: IsScriptCall) -> Self {
            DeployProxyV0Calls::IsScript(var)
        }
    }
    impl ::std::convert::From<RunCall> for DeployProxyV0Calls {
        fn from(var: RunCall) -> Self {
            DeployProxyV0Calls::Run(var)
        }
    }
    #[doc = "Container type for all return fields from the `IS_SCRIPT` function with signature `IS_SCRIPT()` and selector `0xf8ccbf47`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        :: ethers :: contract :: EthAbiType,
        :: ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct IsScriptReturn(pub bool);
}
