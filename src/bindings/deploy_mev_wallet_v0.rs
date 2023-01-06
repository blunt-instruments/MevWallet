pub use deploy_mev_wallet_v0::*;
#[allow(clippy::too_many_arguments, non_camel_case_types)]
pub mod deploy_mev_wallet_v0 {
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
    #[doc = "DeployMevWalletV0 was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    # [rustfmt :: skip] const __ABI : & str = "[{\"inputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"implementation\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"IS_SCRIPT\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"run\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setUp\",\"outputs\":[]}]" ;
    #[doc = r" The parsed JSON-ABI of the contract."]
    pub static DEPLOYMEVWALLETV0_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            ethers::core::utils::__serde_json::from_str(__ABI).expect("invalid abi")
        });
    #[doc = r" Bytecode of the #name contract"]
    pub static DEPLOYMEVWALLETV0_BYTECODE: ethers::contract::Lazy<ethers::core::types::Bytes> =
        ethers::contract::Lazy::new(|| {
            "0x6080806040523461002357600160ff19600c541617600c55610c2990816100298239f35b600080fdfe60806040818152600436101561001457600080fd5b600091823560e01c9081630a9254e414610198578163c040622614610069575063f8ccbf471461004357600080fd5b3461006557816003193601126100655760209060ff600c541690519015158152f35b5080fd5b83915034610065578160031936011261006557737109709ecfa91a80626ff3989d68f67f5b1dd12d803b1561019457816004818580946302bf260160e61b83525af1801561018a5761015f575b50908051610a448082019082821067ffffffffffffffff83111761014b579180917f1c8aff950685c2ed4bc3174f3472287b56d9517b9c948127319a09a7a36deac8936101b08339039084f580156101405790516001600160a01b0390911681527f6b880718e499f885d4786ad669ab7b94d23b6c4322d77d1a4f5c7bd8cad6be7d90602090a180f35b5051903d90823e3d90fd5b634e487b7160e01b85526041600452602485fd5b67ffffffffffffffff8111610176578252826100b6565b634e487b7160e01b82526041600452602482fd5b83513d84823e3d90fd5b8280fd5b83346101ac57806003193601126101ac5780f35b80fdfe60a03461010d57600080546001600160a01b03191660ff1781556001600160401b039160408101838111828210176100f95760019160209160405282815201603160f81b815220906040519060208201927f8b73c3c69bb8fe3d512ecc4cf759cc79239f7b179b0ffacaa9a75d522b39400f84527fb057abb08031679648f38e605817690c05a381e6ec987e5d8cc0600a4d2a5786604084015260608301524660808301523060a083015260a0825260c0820193828510908511176100e5575082604052519020608052610931908161011382396080518181816101bd01526106f00152f35b634e487b7160e01b81526041600452602490fd5b634e487b7160e01b83526041600452602483fd5b600080fdfe6080806040526004908136101561001d575b5050361561001b57005b005b600090813560e01c9081638da5cb5b146107b257508063ae604e4514610777578063affed0e014610759578063c4d66de814610713578063dc0c81b5146106d8578063f2fde38b1461062a5763f319867603610011576101603660031901126103eb576100886107d6565b6024359067ffffffffffffffff821161046a573660238301121561046a57818401356100b38161083d565b926100c1604051948561081b565b8184523660248383010111610622578185926024602093018387013784010152606435151591826064350361062657610104359160ff83168303610622576084359261010f60443585610859565b933a4803610611575a956101a8604051809260208201947f5679fb6ec38d3c67731b4def49181a8fbbb334cda5c263b0993e50cfe699d4e8865260018060a01b0388166040840152610120606084015261016d61014084018a61088b565b91604435608085015260a084015260c083015260a43560e083015260c43561010083015260e43561012083015203601f19810183528261081b565b519020906040519161190160f01b60208401527f0000000000000000000000000000000000000000000000000000000000000000602284015260428301526042825281608081011067ffffffffffffffff6080840111176105fe576080808360ff8a948360209701604052825187840120848401521660a08201526101243560c08201526101443560e08201528380520160015afa156105f35784516001600160a01b031680156105e25785546001600160a01b031681036105cb57506001600160a01b0381161561054f5760a4351515806105c0575b6105a75767ffffffffffffffff421667ffffffffffffffff60c435169067ffffffffffffffff60c43560401c1691801515908161059d575b5061058c57818015159182610582575b505061056b5750846044351261054f5760643580610560575b61054f57846044351380610543575b61052a576001548060e43510610513578060e435116104fc57508491829181606435156104e25750602082519201905af46103286108cb565b905b156104ba575060e4357fbcf6a68a2f901be4a23a41b53acd7697893a7e34def4e28acba584da75283b678480a2600160e435018060e435116104a7576001555a820391821161049457813a02913a8304143a1517156104945782939161038f91610859565b90828213156103fe576e8c43efc014746c230049e330039cb391823b156103f95760248492836040519586948593635992bfdd60e01b85528401525af180156103ee576103db57505080f35b6103e4906107f1565b6103eb5780f35b80fd5b6040513d84823e3d90fd5b505050fd5b9181900390600160ff1b81146001166104815760001982050361046e5782906e8c43efc014746c230049e330039cb392833b1561046a57602490836040519586948593637c72e08160e11b85528401525af180156103ee5761045e575080f35b610467906107f1565b80f35b8280fd5b634e487b7160e01b835260118252602483fd5b634e487b7160e01b845260118352602484fd5b634e487b7160e01b835260118452602483fd5b634e487b7160e01b845260118552602484fd5b604051630540acc960e51b81526020818701529081906104de90602483019061088b565b0390fd5b805192506020019034905af16104f66108cb565b9061032a565b866024916040519163299aa73160e01b8352820152fd5b86602491604051916306ac964b60e41b8352820152fd5b604051637388338760e01b815260443581880152602490fd5b506044353414156102ef565b60405163a04d981f60e01b81528690fd5b5060443515156102e0565b86602491604051916308567e5560e01b8352820152fd5b10905081386102c7565b60405163a04d981f60e01b81528890fd5b90508111386102b7565b604051630e90f1ab60e31b815260a43581880152602490fd5b5060a435481161027f565b8660249160405191631960afe160e11b8352820152fd5b60405163a04d981f60e01b81528790fd5b6040513d86823e3d90fd5b634e487b7160e01b875260418852602487fd5b604051632daf442d60e01b81528890fd5b8480fd5b8380fd5b5090346106d45760203660031901126106d4576106456107d6565b825490916001600160a01b03338184161480156106cb575b1561062257831692831515806106c1575b15610622573b61068957506001600160a01b03191617815580f35b60649060206040519162461bcd60e51b8352820152601160248201527027379031b7b73a3930b1ba1037bbb732b960791b6044820152fd5b503084141561066e565b5030331461065d565b5080fd5b50346103eb57806003193601126103eb5760206040517f00000000000000000000000000000000000000000000000000000000000000008152f35b50346103eb5760203660031901126103eb5761072d6107d6565b8154906001600160a01b03908183166106265716906bffffffffffffffffffffffff60a01b1617815580f35b50346103eb57806003193601126103eb576020600154604051908152f35b50346103eb57806003193601126103eb5760206040517f5679fb6ec38d3c67731b4def49181a8fbbb334cda5c263b0993e50cfe699d4e88152f35b9050346106d457816003193601126106d45790546001600160a01b03168152602090f35b600435906001600160a01b03821682036107ec57565b600080fd5b67ffffffffffffffff811161080557604052565b634e487b7160e01b600052604160045260246000fd5b90601f8019910116810190811067ffffffffffffffff82111761080557604052565b67ffffffffffffffff811161080557601f01601f191660200190565b9190916000838201938412911290801582169115161761087557565b634e487b7160e01b600052601160045260246000fd5b919082519283825260005b8481106108b7575050826000602080949584010152601f8019910116010190565b602081830181015184830182015201610896565b3d156108f6573d906108dc8261083d565b916108ea604051938461081b565b82523d6000602084013e565b60609056fea264697066735822122010d53077ca90b1379c011f3e1f8ff552d48f65492269ed9aa9a24e1a807fdc3464736f6c63430008110033a2646970667358221220001a97b0c9a1a8df0e4dac02407fa5e083d8d716fc903dac21a9b1151fa011df64736f6c63430008110033" . parse () . expect ("invalid bytecode")
        });
    pub struct DeployMevWalletV0<M>(ethers::contract::Contract<M>);
    impl<M> Clone for DeployMevWalletV0<M> {
        fn clone(&self) -> Self {
            DeployMevWalletV0(self.0.clone())
        }
    }
    impl<M> std::ops::Deref for DeployMevWalletV0<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> std::fmt::Debug for DeployMevWalletV0<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(DeployMevWalletV0))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ethers::providers::Middleware> DeployMevWalletV0<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(address.into(), DEPLOYMEVWALLETV0_ABI.clone(), client)
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
                DEPLOYMEVWALLETV0_ABI.clone(),
                DEPLOYMEVWALLETV0_BYTECODE.clone().into(),
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
        #[doc = "Gets the contract's `implementation` event"]
        pub fn implementation_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, ImplementationFilter> {
            self.0.event()
        }
        #[doc = r" Returns an [`Event`](#ethers_contract::builders::Event) builder for all events of this contract"]
        pub fn events(&self) -> ethers::contract::builders::Event<M, ImplementationFilter> {
            self.0.event_with_filter(Default::default())
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>>
        for DeployMevWalletV0<M>
    {
        fn from(contract: ethers::contract::Contract<M>) -> Self {
            Self(contract)
        }
    }
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethevent(name = "implementation", abi = "implementation(address)")]
    pub struct ImplementationFilter(pub ethers::core::types::Address);
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
    pub enum DeployMevWalletV0Calls {
        IsScript(IsScriptCall),
        Run(RunCall),
        SetUp(SetUpCall),
    }
    impl ethers::core::abi::AbiDecode for DeployMevWalletV0Calls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::std::result::Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <IsScriptCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(DeployMevWalletV0Calls::IsScript(decoded));
            }
            if let Ok(decoded) = <RunCall as ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(DeployMevWalletV0Calls::Run(decoded));
            }
            if let Ok(decoded) = <SetUpCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(DeployMevWalletV0Calls::SetUp(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for DeployMevWalletV0Calls {
        fn encode(self) -> Vec<u8> {
            match self {
                DeployMevWalletV0Calls::IsScript(element) => element.encode(),
                DeployMevWalletV0Calls::Run(element) => element.encode(),
                DeployMevWalletV0Calls::SetUp(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for DeployMevWalletV0Calls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                DeployMevWalletV0Calls::IsScript(element) => element.fmt(f),
                DeployMevWalletV0Calls::Run(element) => element.fmt(f),
                DeployMevWalletV0Calls::SetUp(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<IsScriptCall> for DeployMevWalletV0Calls {
        fn from(var: IsScriptCall) -> Self {
            DeployMevWalletV0Calls::IsScript(var)
        }
    }
    impl ::std::convert::From<RunCall> for DeployMevWalletV0Calls {
        fn from(var: RunCall) -> Self {
            DeployMevWalletV0Calls::Run(var)
        }
    }
    impl ::std::convert::From<SetUpCall> for DeployMevWalletV0Calls {
        fn from(var: SetUpCall) -> Self {
            DeployMevWalletV0Calls::SetUp(var)
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
