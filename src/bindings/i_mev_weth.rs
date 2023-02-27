pub use i_mev_weth::*;
/// This module was auto-generated with ethers-rs Abigen.
/// More information at: <https://github.com/gakonst/ethers-rs>
#[allow(
    clippy::enum_variant_names,
    clippy::too_many_arguments,
    clippy::upper_case_acronyms,
    clippy::type_complexity,
    dead_code,
    non_camel_case_types,
)]
pub mod i_mev_weth {
    #[rustfmt::skip]
    const __ABI: &str = "[{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"value\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"addMev\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"from\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"value\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"addMev\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"to\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"getMev\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"getMev\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"to\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"value\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"getMev\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"value\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"getMev\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"mev\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]}]";
    ///The parsed JSON ABI of the contract.
    pub static IMEVWETH_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(||
    ::ethers::core::utils::__serde_json::from_str(__ABI).expect("ABI is always valid"));
    pub struct IMevWeth<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for IMevWeth<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for IMevWeth<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for IMevWeth<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for IMevWeth<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(stringify!(IMevWeth)).field(&self.address()).finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> IMevWeth<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    IMEVWETH_ABI.clone(),
                    client,
                ),
            )
        }
        ///Calls the contract's `addMev` (0x5992bfdd) function
        pub fn add_mev(
            &self,
            value: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([89, 146, 191, 221], value)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `addMev` (0xf801cf60) function
        pub fn add_mev_with_from(
            &self,
            from: ::ethers::core::types::Address,
            value: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([248, 1, 207, 96], (from, value))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getMev` (0x848316fb) function
        pub fn get_mev_1(
            &self,
            to: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([132, 131, 22, 251], to)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getMev` (0x99c24631) function
        pub fn get_mev_0(&self) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([153, 194, 70, 49], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getMev` (0xea3e3e3e) function
        pub fn get_mev_3(
            &self,
            to: ::ethers::core::types::Address,
            value: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([234, 62, 62, 62], (to, value))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getMev` (0xf8e5c102) function
        pub fn get_mev_2(
            &self,
            value: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([248, 229, 193, 2], value)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `mev` (0x13c9b33a) function
        pub fn mev(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([19, 201, 179, 58], ())
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for IMevWeth<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Container type for all input parameters for the `addMev` function with signature `addMev(uint256)` and selector `0x5992bfdd`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "addMev", abi = "addMev(uint256)")]
    pub struct AddMevCall {
        pub value: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `addMev` function with signature `addMev(address,uint256)` and selector `0xf801cf60`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "addMev", abi = "addMev(address,uint256)")]
    pub struct AddMevWithFromCall {
        pub from: ::ethers::core::types::Address,
        pub value: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `getMev` function with signature `getMev(address)` and selector `0x848316fb`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "getMev", abi = "getMev(address)")]
    pub struct GetMev1Call {
        pub to: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `getMev` function with signature `getMev()` and selector `0x99c24631`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "getMev", abi = "getMev()")]
    pub struct GetMev0Call;
    ///Container type for all input parameters for the `getMev` function with signature `getMev(address,uint256)` and selector `0xea3e3e3e`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "getMev", abi = "getMev(address,uint256)")]
    pub struct GetMev3Call {
        pub to: ::ethers::core::types::Address,
        pub value: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `getMev` function with signature `getMev(uint256)` and selector `0xf8e5c102`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "getMev", abi = "getMev(uint256)")]
    pub struct GetMev2Call {
        pub value: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `mev` function with signature `mev()` and selector `0x13c9b33a`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "mev", abi = "mev()")]
    pub struct MevCall;
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum IMevWethCalls {
        AddMev(AddMevCall),
        AddMevWithFrom(AddMevWithFromCall),
        GetMev1(GetMev1Call),
        GetMev0(GetMev0Call),
        GetMev3(GetMev3Call),
        GetMev2(GetMev2Call),
        Mev(MevCall),
    }
    impl ::ethers::core::abi::AbiDecode for IMevWethCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded)
                = <AddMevCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::AddMev(decoded));
            }
            if let Ok(decoded)
                = <AddMevWithFromCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::AddMevWithFrom(decoded));
            }
            if let Ok(decoded)
                = <GetMev1Call as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GetMev1(decoded));
            }
            if let Ok(decoded)
                = <GetMev0Call as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GetMev0(decoded));
            }
            if let Ok(decoded)
                = <GetMev3Call as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GetMev3(decoded));
            }
            if let Ok(decoded)
                = <GetMev2Call as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GetMev2(decoded));
            }
            if let Ok(decoded)
                = <MevCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Mev(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for IMevWethCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::AddMev(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::AddMevWithFrom(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetMev1(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetMev0(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetMev3(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetMev2(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Mev(element) => ::ethers::core::abi::AbiEncode::encode(element),
            }
        }
    }
    impl ::core::fmt::Display for IMevWethCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::AddMev(element) => ::core::fmt::Display::fmt(element, f),
                Self::AddMevWithFrom(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetMev1(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetMev0(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetMev3(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetMev2(element) => ::core::fmt::Display::fmt(element, f),
                Self::Mev(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<AddMevCall> for IMevWethCalls {
        fn from(value: AddMevCall) -> Self {
            Self::AddMev(value)
        }
    }
    impl ::core::convert::From<AddMevWithFromCall> for IMevWethCalls {
        fn from(value: AddMevWithFromCall) -> Self {
            Self::AddMevWithFrom(value)
        }
    }
    impl ::core::convert::From<GetMev1Call> for IMevWethCalls {
        fn from(value: GetMev1Call) -> Self {
            Self::GetMev1(value)
        }
    }
    impl ::core::convert::From<GetMev0Call> for IMevWethCalls {
        fn from(value: GetMev0Call) -> Self {
            Self::GetMev0(value)
        }
    }
    impl ::core::convert::From<GetMev3Call> for IMevWethCalls {
        fn from(value: GetMev3Call) -> Self {
            Self::GetMev3(value)
        }
    }
    impl ::core::convert::From<GetMev2Call> for IMevWethCalls {
        fn from(value: GetMev2Call) -> Self {
            Self::GetMev2(value)
        }
    }
    impl ::core::convert::From<MevCall> for IMevWethCalls {
        fn from(value: MevCall) -> Self {
            Self::Mev(value)
        }
    }
    ///Container type for all return fields from the `mev` function with signature `mev()` and selector `0x13c9b33a`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct MevReturn(pub ::ethers::core::types::U256);
}
