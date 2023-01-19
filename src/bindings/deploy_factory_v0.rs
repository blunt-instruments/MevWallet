pub use deploy_factory_v0::*;
#[allow(clippy::too_many_arguments, non_camel_case_types)]
pub mod deploy_factory_v0 {
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
    #[doc = "DeployFactoryV0 was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    # [rustfmt :: skip] const __ABI : & str = "[{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"IS_SCRIPT\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"run\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setUp\",\"outputs\":[]}]" ;
    #[doc = r" The parsed JSON-ABI of the contract."]
    pub static DEPLOYFACTORYV0_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> =
        ::ethers::contract::Lazy::new(|| {
            ::ethers::core::utils::__serde_json::from_str(__ABI).expect("invalid abi")
        });
    #[doc = r" Bytecode of the #name contract"]
    pub static DEPLOYFACTORYV0_BYTECODE: ::ethers::contract::Lazy<::ethers::core::types::Bytes> =
        ::ethers::contract::Lazy::new(|| {
            "0x6080806040523461002357600160ff19600c541617600c556106f990816100298239f35b600080fdfe60806040818152600436101561001457600080fd5b600091823560e01c9081630a9254e4146101e9578163c040622614610087575063f8ccbf471461004357600080fd5b3461008357817ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc3601126100835760209060ff600c541690519015158152f35b5080fd5b8391503461008357817ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc36011261008357737109709ecfa91a80626ff3989d68f67f5b1dd12d803b156101e557816004818580947fafc980400000000000000000000000000000000000000000000000000000000083525af180156101db57610197575b509080516104ce8082019082821067ffffffffffffffff83111761016a579180917fc9a96dc6eafc5b3263d00fa874e4f0ee5ee58937a905a753a98891286f908ef49361021f8339039084f515610160575080f35b51903d90823e3d90fd5b6024857f4e487b710000000000000000000000000000000000000000000000000000000081526041600452fd5b67ffffffffffffffff81116101ae5782528261010b565b6024827f4e487b710000000000000000000000000000000000000000000000000000000081526041600452fd5b83513d84823e3d90fd5b8280fd5b833461021b57807ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc36011261021b5780f35b80fdfe60808060405234610016576104b2908161001c8239f35b600080fdfe6080604052600436101561001257600080fd5b6000803560e01c80630f6304181461009157631d6476051461003357600080fd5b3461008e5760207ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc36011261008e576020610070336004356102b1565b73ffffffffffffffffffffffffffffffffffffffff60405191168152f35b80fd5b503461008e5760407ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc36011261008e576024359073ffffffffffffffffffffffffffffffffffffffff90818316830361008e57506100f36020926004356102b1565b60405191168152f35b507f4e487b7100000000000000000000000000000000000000000000000000000000600052604160045260246000fd5b90601f7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0910116810190811067ffffffffffffffff82111761016d57604052565b6101756100fc565b604052565b604051906020820182811067ffffffffffffffff82111761016d57604052565b91906020808401936000905b600182106101b45750505050565b828060019273ffffffffffffffffffffffffffffffffffffffff8751168152019401910190926101a6565b3d15610246573d9067ffffffffffffffff8211610239575b6040519161022d60207fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0601f840116018461012c565b82523d6000602084013e565b6102416100fc565b6101f7565b606090565b60208082528251818301819052939260005b85811061029d575050507fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0601f8460006040809697860101520116010190565b81810183015184820160400152820161025d565b604080516020810192835273ffffffffffffffffffffffffffffffffffffffff8416818301528181527fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe09391929061030a60608261012c565b519020603283517f3d602880600a3d3981f3363d3d373d3d3d363d6e8eabbe9a46fa87f0d1e41e6281527fa96d505af43d82803e903d91602657fd5bf3000000000000000000000000000060208201526000f59273ffffffffffffffffffffffffffffffffffffffff84161561047c5760009182916103a661038a61017a565b73ffffffffffffffffffffffffffffffffffffffff9093168352565b6103ed855191826103e160208201957fc4d66de80000000000000000000000000000000000000000000000000000000087526024830161019a565b0390810183528261012c565b519082865af16103fb6101df565b901561044457505173ffffffffffffffffffffffffffffffffffffffff821681527e70c3b37cbd33a8f91033a749a5e534a86edc8c2a4c6155b9c53e99ffd5943790602090a190565b61047891519182917f225d0a580000000000000000000000000000000000000000000000000000000083526004830161024b565b0390fd5b600483517fd786d393000000000000000000000000000000000000000000000000000000008152fdfea164736f6c6343000811000aa164736f6c6343000811000a" . parse () . expect ("invalid bytecode")
        });
    pub struct DeployFactoryV0<M>(::ethers::contract::Contract<M>);
    impl<M> Clone for DeployFactoryV0<M> {
        fn clone(&self) -> Self {
            DeployFactoryV0(self.0.clone())
        }
    }
    impl<M> std::ops::Deref for DeployFactoryV0<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> std::fmt::Debug for DeployFactoryV0<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(DeployFactoryV0))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> DeployFactoryV0<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ::ethers::contract::Contract::new(address.into(), DEPLOYFACTORYV0_ABI.clone(), client)
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
                DEPLOYFACTORYV0_ABI.clone(),
                DEPLOYFACTORYV0_BYTECODE.clone().into(),
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
        #[doc = "Calls the contract's `run` (0xc0406226) function"]
        pub fn run(&self) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([192, 64, 98, 38], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setUp` (0x0a9254e4) function"]
        pub fn set_up(&self) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([10, 146, 84, 228], ())
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
        for DeployFactoryV0<M>
    {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self(contract)
        }
    }
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
    #[doc = "Container type for all input parameters for the `run` function with signature `run()` and selector `0xc0406226`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        :: ethers :: contract :: EthCall,
        :: ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "run", abi = "run()")]
    pub struct RunCall;
    #[doc = "Container type for all input parameters for the `setUp` function with signature `setUp()` and selector `0x0a9254e4`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        :: ethers :: contract :: EthCall,
        :: ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "setUp", abi = "setUp()")]
    pub struct SetUpCall;
    #[derive(Debug, Clone, PartialEq, Eq, :: ethers :: contract :: EthAbiType)]
    pub enum DeployFactoryV0Calls {
        IsScript(IsScriptCall),
        Run(RunCall),
        SetUp(SetUpCall),
    }
    impl ::ethers::core::abi::AbiDecode for DeployFactoryV0Calls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::std::result::Result<Self, ::ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <IsScriptCall as ::ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(DeployFactoryV0Calls::IsScript(decoded));
            }
            if let Ok(decoded) = <RunCall as ::ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(DeployFactoryV0Calls::Run(decoded));
            }
            if let Ok(decoded) =
                <SetUpCall as ::ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(DeployFactoryV0Calls::SetUp(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for DeployFactoryV0Calls {
        fn encode(self) -> Vec<u8> {
            match self {
                DeployFactoryV0Calls::IsScript(element) => element.encode(),
                DeployFactoryV0Calls::Run(element) => element.encode(),
                DeployFactoryV0Calls::SetUp(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for DeployFactoryV0Calls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                DeployFactoryV0Calls::IsScript(element) => element.fmt(f),
                DeployFactoryV0Calls::Run(element) => element.fmt(f),
                DeployFactoryV0Calls::SetUp(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<IsScriptCall> for DeployFactoryV0Calls {
        fn from(var: IsScriptCall) -> Self {
            DeployFactoryV0Calls::IsScript(var)
        }
    }
    impl ::std::convert::From<RunCall> for DeployFactoryV0Calls {
        fn from(var: RunCall) -> Self {
            DeployFactoryV0Calls::Run(var)
        }
    }
    impl ::std::convert::From<SetUpCall> for DeployFactoryV0Calls {
        fn from(var: SetUpCall) -> Self {
            DeployFactoryV0Calls::SetUp(var)
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
