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
    pub static DEPLOYFACTORYV0_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            ethers::core::utils::__serde_json::from_str(__ABI).expect("invalid abi")
        });
    #[doc = r" Bytecode of the #name contract"]
    pub static DEPLOYFACTORYV0_BYTECODE: ethers::contract::Lazy<ethers::core::types::Bytes> =
        ethers::contract::Lazy::new(|| {
            "0x6080806040523461002357600160ff19600c541617600c556104f490816100298239f35b600080fdfe60806040818152600436101561001457600080fd5b600091823560e01c9081630a9254e414610168578163c040622614610069575063f8ccbf471461004357600080fd5b3461006557816003193601126100655760209060ff600c541690519015158152f35b5080fd5b83915034610065578160031936011261006557737109709ecfa91a80626ff3989d68f67f5b1dd12d803b1561016457816004818580946302bf260160e61b83525af1801561015a5761012f575b5090805161033f8082019082821067ffffffffffffffff83111761011b57827fc9a96dc6eafc5b3263d00fa874e4f0ee5ee58937a905a753a98891286f908ef49392602092610180833986815203019084f515610111575080f35b51903d90823e3d90fd5b634e487b7160e01b85526041600452602485fd5b67ffffffffffffffff8111610146578252826100b6565b634e487b7160e01b82526041600452602482fd5b83513d84823e3d90fd5b8280fd5b833461017c578060031936011261017c5780f35b80fdfe60a03461007757601f61033f38819003918201601f19168301916001600160401b0383118484101761007c5780849260209460405283398101031261007757516001600160a01b03811681036100775760601b6001600160601b0319166080526040516102ac908161009382396080518161019f0152f35b600080fd5b634e487b7160e01b600052604160045260246000fdfe6080604052600436101561001257600080fd5b6000803560e01c80630f6304181461006757631d6476051461003357600080fd5b3461006457602036600319011261006457602061005233600435610180565b6040516001600160a01b039091168152f35b80fd5b503461006457604036600319011261006457602435906001600160a01b03908183168303610064575061009e602092600435610180565b60405191168152f35b156100ae57565b600080fd5b50634e487b7160e01b600052604160045260246000fd5b90601f8019910116810190811067ffffffffffffffff8211176100ec57604052565b6100f46100b3565b604052565b91906020808401936000905b600182106101135750505050565b83516001600160a01b0316815292820192600191909101908201610105565b3d1561017b573d9067ffffffffffffffff821161016e575b60405191610162601f8201601f1916602001846100ca565b82523d6000602084013e565b6101766100b3565b61014a565b606090565b604051733d602d80600a3d3981f3363d3d373d3d3d363d7360601b81527f000000000000000000000000000000000000000000000000000000000000000060148201526e5af43d82803e903d91602b57fd5bf360881b60288201526000919060379083f5916001600160a01b03808416156102725791809161026293604051916020830183811067ffffffffffffffff821117610265575b60405216815260405161024e81610240602082019463189acdbd60e31b8652602483016100f9565b03601f1981018352826100ca565b519082865af161025c610132565b506100a7565b90565b61026d6100b3565b610218565b8280fdfea26469706673582212207d7e99091e5cd78f21513c2292f0d2ec36f48cf7fa3ab3953b91bce652f2da6764736f6c63430008110033a2646970667358221220d23f86e4d2e9a0d69fb21e66574ca7b7d1d89f2df164d3d0fd14d884b8f00ed664736f6c63430008110033" . parse () . expect ("invalid bytecode")
        });
    pub struct DeployFactoryV0<M>(ethers::contract::Contract<M>);
    impl<M> Clone for DeployFactoryV0<M> {
        fn clone(&self) -> Self {
            DeployFactoryV0(self.0.clone())
        }
    }
    impl<M> std::ops::Deref for DeployFactoryV0<M> {
        type Target = ethers::contract::Contract<M>;
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
    impl<M: ethers::providers::Middleware> DeployFactoryV0<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(address.into(), DEPLOYFACTORYV0_ABI.clone(), client)
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
                DEPLOYFACTORYV0_ABI.clone(),
                DEPLOYFACTORYV0_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        #[doc = "Calls the contract's `IS_SCRIPT` (0xf8ccbf47) function"]
        pub fn is_script(&self) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([248, 204, 191, 71], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `run` (0xc0406226) function"]
        pub fn run(&self) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([192, 64, 98, 38], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setUp` (0x0a9254e4) function"]
        pub fn set_up(&self) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([10, 146, 84, 228], ())
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>> for DeployFactoryV0<M> {
        fn from(contract: ethers::contract::Contract<M>) -> Self {
            Self(contract)
        }
    }
    #[doc = "Container type for all input parameters for the `IS_SCRIPT` function with signature `IS_SCRIPT()` and selector `0xf8ccbf47`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
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
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
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
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "setUp", abi = "setUp()")]
    pub struct SetUpCall;
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum DeployFactoryV0Calls {
        IsScript(IsScriptCall),
        Run(RunCall),
        SetUp(SetUpCall),
    }
    impl ethers::core::abi::AbiDecode for DeployFactoryV0Calls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::std::result::Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <IsScriptCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(DeployFactoryV0Calls::IsScript(decoded));
            }
            if let Ok(decoded) = <RunCall as ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(DeployFactoryV0Calls::Run(decoded));
            }
            if let Ok(decoded) = <SetUpCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(DeployFactoryV0Calls::SetUp(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for DeployFactoryV0Calls {
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
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct IsScriptReturn(pub bool);
}
