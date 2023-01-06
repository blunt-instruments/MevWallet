pub use i_mev_weth::*;
#[allow(clippy::too_many_arguments, non_camel_case_types)]
pub mod i_mev_weth {
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
    #[doc = "IMevWeth was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    # [rustfmt :: skip] const __ABI : & str = "[{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"value\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"addMev\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"from\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"value\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"addMev\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"to\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"getMev\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"getMev\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"to\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"value\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"getMev\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"value\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"getMev\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"mev\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]}]" ;
    #[doc = r" The parsed JSON-ABI of the contract."]
    pub static IMEVWETH_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            ethers::core::utils::__serde_json::from_str(__ABI).expect("invalid abi")
        });
    pub struct IMevWeth<M>(ethers::contract::Contract<M>);
    impl<M> Clone for IMevWeth<M> {
        fn clone(&self) -> Self {
            IMevWeth(self.0.clone())
        }
    }
    impl<M> std::ops::Deref for IMevWeth<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> std::fmt::Debug for IMevWeth<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(IMevWeth))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ethers::providers::Middleware> IMevWeth<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(address.into(), IMEVWETH_ABI.clone(), client).into()
        }
        #[doc = "Calls the contract's `addMev` (0x5992bfdd) function"]
        pub fn add_mev(
            &self,
            value: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([89, 146, 191, 221], value)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `addMev` (0xf801cf60) function"]
        pub fn add_mev_with_from(
            &self,
            from: ethers::core::types::Address,
            value: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([248, 1, 207, 96], (from, value))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getMev` (0x848316fb) function"]
        pub fn get_mev_1(
            &self,
            to: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([132, 131, 22, 251], to)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getMev` (0x99c24631) function"]
        pub fn get_mev_0(&self) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([153, 194, 70, 49], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getMev` (0xea3e3e3e) function"]
        pub fn get_mev_3(
            &self,
            to: ethers::core::types::Address,
            value: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([234, 62, 62, 62], (to, value))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getMev` (0xf8e5c102) function"]
        pub fn get_mev_2(
            &self,
            value: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([248, 229, 193, 2], value)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `mev` (0x13c9b33a) function"]
        pub fn mev(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([19, 201, 179, 58], ())
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>> for IMevWeth<M> {
        fn from(contract: ethers::contract::Contract<M>) -> Self {
            Self(contract)
        }
    }
    #[doc = "Container type for all input parameters for the `addMev` function with signature `addMev(uint256)` and selector `0x5992bfdd`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "addMev", abi = "addMev(uint256)")]
    pub struct AddMevCall {
        pub value: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `addMev` function with signature `addMev(address,uint256)` and selector `0xf801cf60`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "addMev", abi = "addMev(address,uint256)")]
    pub struct AddMevWithFromCall {
        pub from: ethers::core::types::Address,
        pub value: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `getMev` function with signature `getMev(address)` and selector `0x848316fb`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "getMev", abi = "getMev(address)")]
    pub struct GetMev1Call {
        pub to: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `getMev` function with signature `getMev()` and selector `0x99c24631`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "getMev", abi = "getMev()")]
    pub struct GetMev0Call;
    #[doc = "Container type for all input parameters for the `getMev` function with signature `getMev(address,uint256)` and selector `0xea3e3e3e`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "getMev", abi = "getMev(address,uint256)")]
    pub struct GetMev3Call {
        pub to: ethers::core::types::Address,
        pub value: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `getMev` function with signature `getMev(uint256)` and selector `0xf8e5c102`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "getMev", abi = "getMev(uint256)")]
    pub struct GetMev2Call {
        pub value: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `mev` function with signature `mev()` and selector `0x13c9b33a`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "mev", abi = "mev()")]
    pub struct MevCall;
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum IMevWethCalls {
        AddMev(AddMevCall),
        AddMevWithFrom(AddMevWithFromCall),
        GetMev1(GetMev1Call),
        GetMev0(GetMev0Call),
        GetMev3(GetMev3Call),
        GetMev2(GetMev2Call),
        Mev(MevCall),
    }
    impl ethers::core::abi::AbiDecode for IMevWethCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::std::result::Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) = <AddMevCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IMevWethCalls::AddMev(decoded));
            }
            if let Ok(decoded) =
                <AddMevWithFromCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IMevWethCalls::AddMevWithFrom(decoded));
            }
            if let Ok(decoded) =
                <GetMev1Call as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IMevWethCalls::GetMev1(decoded));
            }
            if let Ok(decoded) =
                <GetMev0Call as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IMevWethCalls::GetMev0(decoded));
            }
            if let Ok(decoded) =
                <GetMev3Call as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IMevWethCalls::GetMev3(decoded));
            }
            if let Ok(decoded) =
                <GetMev2Call as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IMevWethCalls::GetMev2(decoded));
            }
            if let Ok(decoded) = <MevCall as ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(IMevWethCalls::Mev(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for IMevWethCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                IMevWethCalls::AddMev(element) => element.encode(),
                IMevWethCalls::AddMevWithFrom(element) => element.encode(),
                IMevWethCalls::GetMev1(element) => element.encode(),
                IMevWethCalls::GetMev0(element) => element.encode(),
                IMevWethCalls::GetMev3(element) => element.encode(),
                IMevWethCalls::GetMev2(element) => element.encode(),
                IMevWethCalls::Mev(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for IMevWethCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                IMevWethCalls::AddMev(element) => element.fmt(f),
                IMevWethCalls::AddMevWithFrom(element) => element.fmt(f),
                IMevWethCalls::GetMev1(element) => element.fmt(f),
                IMevWethCalls::GetMev0(element) => element.fmt(f),
                IMevWethCalls::GetMev3(element) => element.fmt(f),
                IMevWethCalls::GetMev2(element) => element.fmt(f),
                IMevWethCalls::Mev(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<AddMevCall> for IMevWethCalls {
        fn from(var: AddMevCall) -> Self {
            IMevWethCalls::AddMev(var)
        }
    }
    impl ::std::convert::From<AddMevWithFromCall> for IMevWethCalls {
        fn from(var: AddMevWithFromCall) -> Self {
            IMevWethCalls::AddMevWithFrom(var)
        }
    }
    impl ::std::convert::From<GetMev1Call> for IMevWethCalls {
        fn from(var: GetMev1Call) -> Self {
            IMevWethCalls::GetMev1(var)
        }
    }
    impl ::std::convert::From<GetMev0Call> for IMevWethCalls {
        fn from(var: GetMev0Call) -> Self {
            IMevWethCalls::GetMev0(var)
        }
    }
    impl ::std::convert::From<GetMev3Call> for IMevWethCalls {
        fn from(var: GetMev3Call) -> Self {
            IMevWethCalls::GetMev3(var)
        }
    }
    impl ::std::convert::From<GetMev2Call> for IMevWethCalls {
        fn from(var: GetMev2Call) -> Self {
            IMevWethCalls::GetMev2(var)
        }
    }
    impl ::std::convert::From<MevCall> for IMevWethCalls {
        fn from(var: MevCall) -> Self {
            IMevWethCalls::Mev(var)
        }
    }
    #[doc = "Container type for all return fields from the `mev` function with signature `mev()` and selector `0x13c9b33a`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct MevReturn(pub ethers::core::types::U256);
}
