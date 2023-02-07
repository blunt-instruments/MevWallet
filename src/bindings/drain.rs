pub use drain::*;
#[allow(clippy::too_many_arguments, non_camel_case_types)]
pub mod drain {
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
    #[doc = "Drain was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    # [rustfmt :: skip] const __ABI : & str = "[{\"inputs\":[{\"internalType\":\"address\",\"name\":\"asset\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"to\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"approveAll\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"to\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"drain\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"to\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"drainAndApprove\",\"outputs\":[]}]" ;
    #[doc = r" The parsed JSON-ABI of the contract."]
    pub static DRAIN_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> =
        ::ethers::contract::Lazy::new(|| {
            ::ethers::core::utils::__serde_json::from_str(__ABI).expect("invalid abi")
        });
    #[doc = r" Bytecode of the #name contract"]
    pub static DRAIN_BYTECODE: ::ethers::contract::Lazy<::ethers::core::types::Bytes> =
        ::ethers::contract::Lazy::new(|| {
            "0x608080604052346100165761034b908161001c8239f35b600080fdfe6080604090808252600436101561001557600080fd5b600091823560e01c9182630a4432761461020257508163101ae65e1461008a575063ece531321461004557600080fd5b346100875760207ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc3601126100875761008461007f6102d0565b6102f8565b80f35b80fd5b9050346101bf5760207ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc3601126101bf576100c36102d0565b906100cd826102f8565b73ffffffffffffffffffffffffffffffffffffffff8151927f095ea7b30000000000000000000000000000000000000000000000000000000084521660048301527fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff6024830152602082604481866e8c43efc014746c230049e330039cb35af180156101f85761015b578280f35b6020903d82116101f0575b7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0601f83011683019083821067ffffffffffffffff8311176101c357526020908201829003126101bf5751801515036100875738808280f35b5080fd5b6024857f4e487b710000000000000000000000000000000000000000000000000000000081526041600452fd5b3d9150610166565b81513d85823e3d90fd5b9150346102cc57807ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc3601126102cc578261023b6102d0565b6024359073ffffffffffffffffffffffffffffffffffffffff918281168091036102c857602093869360449385937f095ea7b300000000000000000000000000000000000000000000000000000000855260048501527fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff6024850152165af180156101f85761015b578280f35b8380fd5b8280fd5b6004359073ffffffffffffffffffffffffffffffffffffffff821682036102f357565b600080fd5b600080808093479082908215610334575b73ffffffffffffffffffffffffffffffffffffffff1690f11561032857565b6040513d6000823e3d90fd5b6108fc915061030956fea164736f6c6343000811000a" . parse () . expect ("invalid bytecode")
        });
    pub struct Drain<M>(::ethers::contract::Contract<M>);
    impl<M> Clone for Drain<M> {
        fn clone(&self) -> Self {
            Drain(self.0.clone())
        }
    }
    impl<M> std::ops::Deref for Drain<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> std::fmt::Debug for Drain<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(Drain))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> Drain<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ::ethers::contract::Contract::new(address.into(), DRAIN_ABI.clone(), client).into()
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
                DRAIN_ABI.clone(),
                DRAIN_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        #[doc = "Calls the contract's `approveAll` (0x0a443276) function"]
        pub fn approve_all(
            &self,
            asset: ::ethers::core::types::Address,
            to: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([10, 68, 50, 118], (asset, to))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `drain` (0xece53132) function"]
        pub fn drain(
            &self,
            to: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([236, 229, 49, 50], to)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `drainAndApprove` (0x101ae65e) function"]
        pub fn drain_and_approve(
            &self,
            to: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([16, 26, 230, 94], to)
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>> for Drain<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self(contract)
        }
    }
    #[doc = "Container type for all input parameters for the `approveAll` function with signature `approveAll(address,address)` and selector `0x0a443276`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        :: ethers :: contract :: EthCall,
        :: ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "approveAll", abi = "approveAll(address,address)")]
    pub struct ApproveAllCall {
        pub asset: ::ethers::core::types::Address,
        pub to: ::ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `drain` function with signature `drain(address)` and selector `0xece53132`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        :: ethers :: contract :: EthCall,
        :: ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "drain", abi = "drain(address)")]
    pub struct DrainCall {
        pub to: ::ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `drainAndApprove` function with signature `drainAndApprove(address)` and selector `0x101ae65e`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        :: ethers :: contract :: EthCall,
        :: ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "drainAndApprove", abi = "drainAndApprove(address)")]
    pub struct DrainAndApproveCall {
        pub to: ::ethers::core::types::Address,
    }
    #[derive(Debug, Clone, PartialEq, Eq, :: ethers :: contract :: EthAbiType)]
    pub enum DrainCalls {
        ApproveAll(ApproveAllCall),
        Drain(DrainCall),
        DrainAndApprove(DrainAndApproveCall),
    }
    impl ::ethers::core::abi::AbiDecode for DrainCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::std::result::Result<Self, ::ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <ApproveAllCall as ::ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(DrainCalls::ApproveAll(decoded));
            }
            if let Ok(decoded) =
                <DrainCall as ::ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(DrainCalls::Drain(decoded));
            }
            if let Ok(decoded) =
                <DrainAndApproveCall as ::ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(DrainCalls::DrainAndApprove(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for DrainCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                DrainCalls::ApproveAll(element) => element.encode(),
                DrainCalls::Drain(element) => element.encode(),
                DrainCalls::DrainAndApprove(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for DrainCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                DrainCalls::ApproveAll(element) => element.fmt(f),
                DrainCalls::Drain(element) => element.fmt(f),
                DrainCalls::DrainAndApprove(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<ApproveAllCall> for DrainCalls {
        fn from(var: ApproveAllCall) -> Self {
            DrainCalls::ApproveAll(var)
        }
    }
    impl ::std::convert::From<DrainCall> for DrainCalls {
        fn from(var: DrainCall) -> Self {
            DrainCalls::Drain(var)
        }
    }
    impl ::std::convert::From<DrainAndApproveCall> for DrainCalls {
        fn from(var: DrainAndApproveCall) -> Self {
            DrainCalls::DrainAndApprove(var)
        }
    }
}
