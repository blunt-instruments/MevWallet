pub use deploy_impl_v1::*;
#[allow(clippy::too_many_arguments, non_camel_case_types)]
pub mod deploy_impl_v1 {
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
    #[doc = "DeployImplV1 was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    # [rustfmt :: skip] const __ABI : & str = "[{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"h\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"IS_SCRIPT\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"run\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setUp\",\"outputs\":[]}]" ;
    #[doc = r" The parsed JSON-ABI of the contract."]
    pub static DEPLOYIMPLV1_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> =
        ::ethers::contract::Lazy::new(|| {
            ::ethers::core::utils::__serde_json::from_str(__ABI).expect("invalid abi")
        });
    #[doc = r" Bytecode of the #name contract"]
    pub static DEPLOYIMPLV1_BYTECODE: ::ethers::contract::Lazy<::ethers::core::types::Bytes> =
        ::ethers::contract::Lazy::new(|| {
            "0x6080806040523461002357600160ff19600c541617600c556110fd90816100298239f35b600080fdfe60806040818152600436101561001457600080fd5b600091823560e01c9081630a9254e414610288578163c040622614610087575063f8ccbf471461004357600080fd5b3461008357817ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc3601126100835760209060ff600c541690519015158152f35b5080fd5b8391503461008357817ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc36011261008357737109709ecfa91a80626ff3989d68f67f5b1dd12d803b1561028457816004818580947fafc980400000000000000000000000000000000000000000000000000000000083525af1801561027a57610236575b50908051610e339182820167ffffffffffffffff928082108483111761020957807f13061f1c1bbc52beabac07a60520c45cf18845c97844375d0e2d4023552cf4ac916102be9387858339039087f5156101ff578151603f85017fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0168101938411818510176101d0577fc05aa43757f73b75f94842aae43554395cf19d66984096e13983ad73265d12dc9460209484528082528482019283395190209051908152a180f35b7f4e487b7100000000000000000000000000000000000000000000000000000000600052604160045260246000fd5b81513d86823e3d90fd5b6024867f4e487b710000000000000000000000000000000000000000000000000000000081526041600452fd5b67ffffffffffffffff811161024d5782528261010b565b6024827f4e487b710000000000000000000000000000000000000000000000000000000081526041600452fd5b83513d84823e3d90fd5b8280fd5b83346102ba57807ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc3601126102ba5780f35b80fdfe6080806040523461002957600180546001600160a01b03191660ff179055610e04908161002f8239f35b600080fdfe608080604052600436101561001a575b50361561001857005b005b600090813560e01c9081638da5cb5b14610be157508063ae604e4514610b88578063affed0e014610b4c578063c4d66de8146109a6578063dc0c81b51461096b578063f2fde38b146108b15763f31986760361000f576101607ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc36011261049c576100a3610c31565b906024359167ffffffffffffffff83116108ad57366023840112156108ad5782600401356100d081610cdd565b936100de6040519586610c9c565b81855236602483830101116108a557818492602460209301838801378501015260643515159283606435036108a957610104359060ff821682036108a5576084359461012a3487610d7c565b923a480361087b575a968351602085012092604051937ffd4c9c9ceea85482c3671626f7b1c65bc2230325b86dbcb0f971327c3062c26a602086015273ffffffffffffffffffffffffffffffffffffffff881660408601526060850152604435608085015260a084015260c083015260a43560e083015260c43561010083015261012060e4358184015282528161014081011067ffffffffffffffff6101408401111761084e57610140820160408190528251602084012087547f19010000000000000000000000000000000000000000000000000000000000006101608601526101628501526101828401526042815267ffffffffffffffff6101c0840190811191111761084e5760806101c08360ff8994836020970160405261014083015161016084012084840152166101e082015261012435610200820152610144356102208201528380520160015afa156108435773ffffffffffffffffffffffffffffffffffffffff845116801561075b5773ffffffffffffffffffffffffffffffffffffffff600154168103610812575073ffffffffffffffffffffffffffffffffffffffff83161561075b5760a435151580610807575b6107d55767ffffffffffffffff421667ffffffffffffffff60c435169067ffffffffffffffff60c43560401c169180151590816107cb575b5061075b578180151591826107c1575b50506107905750836044351261075b5760643580610785575b61075b573447036044358110801580610752575b610721576106e5575b506002548060e435106106b4578060e4351161068357508380917fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff9485600255816064356000146106675750602082519201905af46103bd610dc7565b905b156105d45750600160e435018060e4351161057a5760025560e4357fbcf6a68a2f901be4a23a41b53acd7697893a7e34def4e28acba584da75283b678480a25a84039384116105a757833a02933a8504143a1517156105a757829361042391610d7c565b90828213156104ae57506e8c43efc014746c230049e330039cb390813b156104aa5782916024839260405194859384927f5992bfdd00000000000000000000000000000000000000000000000000000000845260048401525af1801561049f5761048c57505080f35b61049590610c59565b61049c5780f35b80fd5b6040513d84823e3d90fd5b5050fd5b91819003917f8000000000000000000000000000000000000000000000000000000000000000821460011661057a5782050361054d5781906e8c43efc014746c230049e330039cb3803b156104aa576024839260405194859384927ff8e5c10200000000000000000000000000000000000000000000000000000000845260048401525af1801561049f57610541575080f35b61054a90610c59565b80f35b6024827f4e487b710000000000000000000000000000000000000000000000000000000081526011600452fd5b6024847f4e487b710000000000000000000000000000000000000000000000000000000081526011600452fd5b6024837f4e487b710000000000000000000000000000000000000000000000000000000081526011600452fd5b839060405180927fa815992000000000000000000000000000000000000000000000000000000000825260206004830152825192836024840152815b84811061064f575050601f837fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe092604480968601015201168101030190fd5b60208282018101516044888401015286945001610610565b8051925060200190604435905af161067d610dc7565b906103bf565b602490604051907f299aa7310000000000000000000000000000000000000000000000000000000082526004820152fd5b602490604051907f6ac964b00000000000000000000000000000000000000000000000000000000082526004820152fd5b6044350334811461036057602490604051907f738833870000000000000000000000000000000000000000000000000000000082526004820152fd5b602486604051907f738833870000000000000000000000000000000000000000000000000000000082526004820152fd5b50341515610357565b60046040517fa04d981f000000000000000000000000000000000000000000000000000000008152fd5b506044351515610343565b602490604051907f08567e550000000000000000000000000000000000000000000000000000000082526004820152fd5b109050813861032a565b905081113861031a565b60246040517f74878d5800000000000000000000000000000000000000000000000000000000815260a4356004820152fd5b5060a43548116102e2565b602490604051907f32c15fc20000000000000000000000000000000000000000000000000000000082526004820152fd5b6040513d85823e3d90fd5b6024867f4e487b710000000000000000000000000000000000000000000000000000000081526041600452fd5b60046040517f2daf442d000000000000000000000000000000000000000000000000000000008152fd5b8380fd5b8280fd5b5080fd5b503461049c5760207ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc36011261049c576108e9610c31565b6001549073ffffffffffffffffffffffffffffffffffffffff80831633148015610962575b156108a55781169182151580610958575b156108a5576109507fffffffffffffffffffffffff0000000000000000000000000000000000000000923b15610d17565b161760015580f35b503083141561091f565b5030331461090e565b503461049c57807ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc36011261049c5760209054604051908152f35b503461049c5760207ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc36011261049c576109de610c31565b6001549073ffffffffffffffffffffffffffffffffffffffff8083166108a55781610a2b7fffffffffffffffffffffffff0000000000000000000000000000000000000000933b15610d17565b16911617600155604051604081019067ffffffffffffffff9181811083821117610b1f577f31000000000000000000000000000000000000000000000000000000000000009160209160405260018152015260405160208101917f8b73c3c69bb8fe3d512ecc4cf759cc79239f7b179b0ffacaa9a75d522b39400f83527fb057abb08031679648f38e605817690c05a381e6ec987e5d8cc0600a4d2a578660408301527fc89efdaa54c0f20c7adf612882df0950f5a951637e0307cdcb4c672f298b8bc660608301524660808301523060a083015260a0825260c082019082821090821117610b1f57604052519020815580f35b6024847f4e487b710000000000000000000000000000000000000000000000000000000081526041600452fd5b503461049c57807ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc36011261049c576020600254604051908152f35b503461049c57807ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc36011261049c5760206040517ffd4c9c9ceea85482c3671626f7b1c65bc2230325b86dbcb0f971327c3062c26a8152f35b9050346108ad57817ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc3601126108ad5760209073ffffffffffffffffffffffffffffffffffffffff600154168152f35b6004359073ffffffffffffffffffffffffffffffffffffffff82168203610c5457565b600080fd5b67ffffffffffffffff8111610c6d57604052565b7f4e487b7100000000000000000000000000000000000000000000000000000000600052604160045260246000fd5b90601f7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0910116810190811067ffffffffffffffff821117610c6d57604052565b67ffffffffffffffff8111610c6d57601f017fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe01660200190565b15610d1e57565b60646040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152601160248201527f4e6f20636f6e7472616374206f776e65720000000000000000000000000000006044820152fd5b91909160008382019384129112908015821691151617610d9857565b7f4e487b7100000000000000000000000000000000000000000000000000000000600052601160045260246000fd5b3d15610df2573d90610dd882610cdd565b91610de66040519384610c9c565b82523d6000602084013e565b60609056fea164736f6c6343000811000aa164736f6c6343000811000a" . parse () . expect ("invalid bytecode")
        });
    pub struct DeployImplV1<M>(::ethers::contract::Contract<M>);
    impl<M> Clone for DeployImplV1<M> {
        fn clone(&self) -> Self {
            DeployImplV1(self.0.clone())
        }
    }
    impl<M> std::ops::Deref for DeployImplV1<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> std::fmt::Debug for DeployImplV1<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(DeployImplV1))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> DeployImplV1<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ::ethers::contract::Contract::new(address.into(), DEPLOYIMPLV1_ABI.clone(), client)
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
                DEPLOYIMPLV1_ABI.clone(),
                DEPLOYIMPLV1_BYTECODE.clone().into(),
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
        #[doc = "Gets the contract's `h` event"]
        pub fn h_filter(&self) -> ::ethers::contract::builders::Event<M, HFilter> {
            self.0.event()
        }
        #[doc = r" Returns an [`Event`](#ethers_contract::builders::Event) builder for all events of this contract"]
        pub fn events(&self) -> ::ethers::contract::builders::Event<M, HFilter> {
            self.0.event_with_filter(Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>> for DeployImplV1<M> {
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
    #[ethevent(name = "h", abi = "h(bytes32)")]
    pub struct HFilter(pub [u8; 32]);
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
    pub enum DeployImplV1Calls {
        IsScript(IsScriptCall),
        Run(RunCall),
        SetUp(SetUpCall),
    }
    impl ::ethers::core::abi::AbiDecode for DeployImplV1Calls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::std::result::Result<Self, ::ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <IsScriptCall as ::ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(DeployImplV1Calls::IsScript(decoded));
            }
            if let Ok(decoded) = <RunCall as ::ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(DeployImplV1Calls::Run(decoded));
            }
            if let Ok(decoded) =
                <SetUpCall as ::ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(DeployImplV1Calls::SetUp(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for DeployImplV1Calls {
        fn encode(self) -> Vec<u8> {
            match self {
                DeployImplV1Calls::IsScript(element) => element.encode(),
                DeployImplV1Calls::Run(element) => element.encode(),
                DeployImplV1Calls::SetUp(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for DeployImplV1Calls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                DeployImplV1Calls::IsScript(element) => element.fmt(f),
                DeployImplV1Calls::Run(element) => element.fmt(f),
                DeployImplV1Calls::SetUp(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<IsScriptCall> for DeployImplV1Calls {
        fn from(var: IsScriptCall) -> Self {
            DeployImplV1Calls::IsScript(var)
        }
    }
    impl ::std::convert::From<RunCall> for DeployImplV1Calls {
        fn from(var: RunCall) -> Self {
            DeployImplV1Calls::Run(var)
        }
    }
    impl ::std::convert::From<SetUpCall> for DeployImplV1Calls {
        fn from(var: SetUpCall) -> Self {
            DeployImplV1Calls::SetUp(var)
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
