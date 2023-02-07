pub use mwb::*;
#[allow(clippy::too_many_arguments, non_camel_case_types)]
pub mod mwb {
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
    #[doc = "MWB was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    # [rustfmt :: skip] const __ABI : & str = "[{\"inputs\":[{\"internalType\":\"address\",\"name\":\"at\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"etch\",\"outputs\":[]}]" ;
    #[doc = r" The parsed JSON-ABI of the contract."]
    pub static MWB_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> =
        ::ethers::contract::Lazy::new(|| {
            ::ethers::core::utils::__serde_json::from_str(__ABI).expect("invalid abi")
        });
    #[doc = r" Bytecode of the #name contract"]
    pub static MWB_BYTECODE: ::ethers::contract::Lazy<::ethers::core::types::Bytes> =
        ::ethers::contract::Lazy::new(|| {
            "0x608080604052346100165761037c908161001c8239f35b600080fdfe60806040908082526004918236101561001757600080fd5b600092833560e01c6377c243eb1461002e57600080fd5b346101e357836020807ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc3601126102d85782359073ffffffffffffffffffffffffffffffffffffffff82168092036102d4577f4d7baf060000000000000000000000000000000000000000000000000000000086528084870152600360248701527f4d574200000000000000000000000000000000000000000000000000000000006044870152737109709ecfa91a80626ff3989d68f67f5b1dd12d908387606481855afa9687156102ca5784976101e7575b50813b156101e35783606461017e947fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0601f8a519b8c98899788957fb4d6c7820000000000000000000000000000000000000000000000000000000087528d8701528d60248701528251928391826044890152888801910161034c565b011681010301925af180156101d957610195578380f35b67ffffffffffffffff83116101ad5750523880808380f35b8360416024927f4e487b7100000000000000000000000000000000000000000000000000000000835252fd5b82513d86823e3d90fd5b8380fd5b9192935095503d8088833e6101fc81836102dc565b81019086818303126102c657805167ffffffffffffffff918282116102c257019082601f830112156102be5781519081116102925790889493929187519261026b8a7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0601f85011601856102dc565b81845289828401011161028e576102879189808501910161034c565b9538610101565b8580fd5b6024896041887f4e487b7100000000000000000000000000000000000000000000000000000000835252fd5b8880fd5b8980fd5b8780fd5b86513d86823e3d90fd5b8280fd5b5080fd5b90601f7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0910116810190811067ffffffffffffffff82111761031d57604052565b7f4e487b7100000000000000000000000000000000000000000000000000000000600052604160045260246000fd5b60005b83811061035f5750506000910152565b818101518382015260200161034f56fea164736f6c6343000811000a" . parse () . expect ("invalid bytecode")
        });
    pub struct MWB<M>(::ethers::contract::Contract<M>);
    impl<M> Clone for MWB<M> {
        fn clone(&self) -> Self {
            MWB(self.0.clone())
        }
    }
    impl<M> std::ops::Deref for MWB<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> std::fmt::Debug for MWB<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(MWB))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> MWB<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ::ethers::contract::Contract::new(address.into(), MWB_ABI.clone(), client).into()
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
                MWB_ABI.clone(),
                MWB_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        #[doc = "Calls the contract's `etch` (0x77c243eb) function"]
        pub fn etch(
            &self,
            at: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([119, 194, 67, 235], at)
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>> for MWB<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self(contract)
        }
    }
    #[doc = "Container type for all input parameters for the `etch` function with signature `etch(address)` and selector `0x77c243eb`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        :: ethers :: contract :: EthCall,
        :: ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "etch", abi = "etch(address)")]
    pub struct EtchCall {
        pub at: ::ethers::core::types::Address,
    }
}
