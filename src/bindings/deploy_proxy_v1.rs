pub use deploy_proxy_v1::*;
#[allow(clippy::too_many_arguments, non_camel_case_types)]
pub mod deploy_proxy_v1 {
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
    #[doc = "DeployProxyV1 was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    # [rustfmt :: skip] const __ABI : & str = "[{\"inputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"proxy\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"IS_SCRIPT\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"salt\",\"type\":\"bytes32\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"run\",\"outputs\":[]}]" ;
    #[doc = r" The parsed JSON-ABI of the contract."]
    pub static DEPLOYPROXYV1_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> =
        ::ethers::contract::Lazy::new(|| {
            ::ethers::core::utils::__serde_json::from_str(__ABI).expect("invalid abi")
        });
    #[doc = r" Bytecode of the #name contract"]
    pub static DEPLOYPROXYV1_BYTECODE: ::ethers::contract::Lazy<::ethers::core::types::Bytes> =
        ::ethers::contract::Lazy::new(|| {
            "0x6080806040523461003d57600c80546001600160a81b031916749248b5e672e1880af34068c0fae18d30c26d05fb011790556102c690816100438239f35b600080fdfe6080604081815260048036101561001557600080fd5b600092833560e01c918263ef6ac0f01461007e57505063f8ccbf471461003a57600080fd5b3461007a57817ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc36011261007a5760209060ff600c541690519015158152f35b5080fd5b909150346102b557602091827ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc3601126102205784737109709ecfa91a80626ff3989d68f67f5b1dd12d803b1561007a5783838381937fafc980400000000000000000000000000000000000000000000000000000000083525af180156102ab57610264575b5073ffffffffffffffffffffffffffffffffffffffff92848385600c5460081c1660248451809481937f1d6476050000000000000000000000000000000000000000000000000000000083528835898401525af192831561025a578693610193575b505051921682527f06713c3e25d77cfc819deb809c193ba05adfd180f864702d7d992371e78e3e7f91a180f35b9091925083903d8511610252575b7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0601f83011683019083821067ffffffffffffffff83111761022457508491839185528101031261022057519280841684036102205792907f06713c3e25d77cfc819deb809c193ba05adfd180f864702d7d992371e78e3e7f38610166565b8480fd5b6041907f4e487b71000000000000000000000000000000000000000000000000000000006000525260246000fd5b3d91506101a1565b82513d88823e3d90fd5b67ffffffffffffffff819592951161027f5783529238610104565b6024826041877f4e487b7100000000000000000000000000000000000000000000000000000000835252fd5b84513d87823e3d90fd5b8380fdfea164736f6c6343000811000a" . parse () . expect ("invalid bytecode")
        });
    pub struct DeployProxyV1<M>(::ethers::contract::Contract<M>);
    impl<M> Clone for DeployProxyV1<M> {
        fn clone(&self) -> Self {
            DeployProxyV1(self.0.clone())
        }
    }
    impl<M> std::ops::Deref for DeployProxyV1<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> std::fmt::Debug for DeployProxyV1<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(DeployProxyV1))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> DeployProxyV1<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ::ethers::contract::Contract::new(address.into(), DEPLOYPROXYV1_ABI.clone(), client)
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
                DEPLOYPROXYV1_ABI.clone(),
                DEPLOYPROXYV1_BYTECODE.clone().into(),
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
        pub fn run(&self, salt: [u8; 32]) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([239, 106, 192, 240], salt)
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
        for DeployProxyV1<M>
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
    pub struct RunCall {
        pub salt: [u8; 32],
    }
    #[derive(Debug, Clone, PartialEq, Eq, :: ethers :: contract :: EthAbiType)]
    pub enum DeployProxyV1Calls {
        IsScript(IsScriptCall),
        Run(RunCall),
    }
    impl ::ethers::core::abi::AbiDecode for DeployProxyV1Calls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::std::result::Result<Self, ::ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <IsScriptCall as ::ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(DeployProxyV1Calls::IsScript(decoded));
            }
            if let Ok(decoded) = <RunCall as ::ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(DeployProxyV1Calls::Run(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for DeployProxyV1Calls {
        fn encode(self) -> Vec<u8> {
            match self {
                DeployProxyV1Calls::IsScript(element) => element.encode(),
                DeployProxyV1Calls::Run(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for DeployProxyV1Calls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                DeployProxyV1Calls::IsScript(element) => element.fmt(f),
                DeployProxyV1Calls::Run(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<IsScriptCall> for DeployProxyV1Calls {
        fn from(var: IsScriptCall) -> Self {
            DeployProxyV1Calls::IsScript(var)
        }
    }
    impl ::std::convert::From<RunCall> for DeployProxyV1Calls {
        fn from(var: RunCall) -> Self {
            DeployProxyV1Calls::Run(var)
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
