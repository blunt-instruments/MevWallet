pub use deploy_proxy_v0::*;
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
pub mod deploy_proxy_v0 {
    #[rustfmt::skip]
    const __ABI: &str = "[{\"inputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"proxy\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"IS_SCRIPT\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\",\"components\":[]}],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"run\",\"outputs\":[]}]";
    ///The parsed JSON ABI of the contract.
    pub static DEPLOYPROXYV0_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(||
    ::ethers::core::utils::__serde_json::from_str(__ABI).expect("ABI is always valid"));
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = &[
        96,
        128,
        128,
        96,
        64,
        82,
        52,
        97,
        0,
        61,
        87,
        96,
        12,
        128,
        84,
        96,
        1,
        96,
        1,
        96,
        168,
        27,
        3,
        25,
        22,
        116,
        68,
        69,
        68,
        165,
        75,
        81,
        147,
        186,
        109,
        26,
        60,
        249,
        200,
        62,
        225,
        36,
        34,
        182,
        168,
        36,
        1,
        23,
        144,
        85,
        97,
        1,
        24,
        144,
        129,
        97,
        0,
        67,
        130,
        57,
        243,
        91,
        96,
        0,
        128,
        253,
        254,
        96,
        128,
        128,
        96,
        64,
        82,
        96,
        4,
        54,
        16,
        21,
        97,
        0,
        19,
        87,
        96,
        0,
        128,
        253,
        91,
        96,
        0,
        144,
        129,
        53,
        96,
        224,
        28,
        144,
        129,
        99,
        239,
        106,
        192,
        240,
        20,
        97,
        0,
        122,
        87,
        80,
        99,
        248,
        204,
        191,
        71,
        20,
        97,
        0,
        55,
        87,
        96,
        0,
        128,
        253,
        91,
        52,
        97,
        0,
        119,
        87,
        128,
        127,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        252,
        54,
        1,
        18,
        97,
        0,
        119,
        87,
        96,
        32,
        96,
        255,
        96,
        12,
        84,
        22,
        96,
        64,
        81,
        144,
        21,
        21,
        129,
        82,
        243,
        91,
        128,
        253,
        91,
        144,
        80,
        52,
        97,
        1,
        7,
        87,
        96,
        32,
        127,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        252,
        54,
        1,
        18,
        97,
        1,
        7,
        87,
        128,
        127,
        8,
        195,
        121,
        160,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        96,
        100,
        146,
        82,
        96,
        32,
        96,
        4,
        130,
        1,
        82,
        96,
        31,
        96,
        36,
        130,
        1,
        82,
        127,
        77,
        101,
        118,
        87,
        97,
        108,
        108,
        101,
        116,
        86,
        48,
        32,
        117,
        115,
        101,
        32,
        110,
        111,
        116,
        32,
        114,
        101,
        99,
        111,
        109,
        109,
        101,
        110,
        100,
        101,
        100,
        0,
        96,
        68,
        130,
        1,
        82,
        253,
        91,
        80,
        128,
        253,
        254,
        161,
        100,
        115,
        111,
        108,
        99,
        67,
        0,
        8,
        17,
        0,
        10,
    ];
    ///The bytecode of the contract.
    pub static DEPLOYPROXYV0_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = &[
        96,
        128,
        128,
        96,
        64,
        82,
        96,
        4,
        54,
        16,
        21,
        97,
        0,
        19,
        87,
        96,
        0,
        128,
        253,
        91,
        96,
        0,
        144,
        129,
        53,
        96,
        224,
        28,
        144,
        129,
        99,
        239,
        106,
        192,
        240,
        20,
        97,
        0,
        122,
        87,
        80,
        99,
        248,
        204,
        191,
        71,
        20,
        97,
        0,
        55,
        87,
        96,
        0,
        128,
        253,
        91,
        52,
        97,
        0,
        119,
        87,
        128,
        127,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        252,
        54,
        1,
        18,
        97,
        0,
        119,
        87,
        96,
        32,
        96,
        255,
        96,
        12,
        84,
        22,
        96,
        64,
        81,
        144,
        21,
        21,
        129,
        82,
        243,
        91,
        128,
        253,
        91,
        144,
        80,
        52,
        97,
        1,
        7,
        87,
        96,
        32,
        127,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        252,
        54,
        1,
        18,
        97,
        1,
        7,
        87,
        128,
        127,
        8,
        195,
        121,
        160,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        96,
        100,
        146,
        82,
        96,
        32,
        96,
        4,
        130,
        1,
        82,
        96,
        31,
        96,
        36,
        130,
        1,
        82,
        127,
        77,
        101,
        118,
        87,
        97,
        108,
        108,
        101,
        116,
        86,
        48,
        32,
        117,
        115,
        101,
        32,
        110,
        111,
        116,
        32,
        114,
        101,
        99,
        111,
        109,
        109,
        101,
        110,
        100,
        101,
        100,
        0,
        96,
        68,
        130,
        1,
        82,
        253,
        91,
        80,
        128,
        253,
        254,
        161,
        100,
        115,
        111,
        108,
        99,
        67,
        0,
        8,
        17,
        0,
        10,
    ];
    ///The deployed bytecode of the contract.
    pub static DEPLOYPROXYV0_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct DeployProxyV0<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for DeployProxyV0<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for DeployProxyV0<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for DeployProxyV0<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for DeployProxyV0<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(stringify!(DeployProxyV0)).field(&self.address()).finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> DeployProxyV0<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    DEPLOYPROXYV0_ABI.clone(),
                    client,
                ),
            )
        }
        /// Constructs the general purpose `Deployer` instance based on the provided constructor arguments and sends it.
        /// Returns a new instance of a deployer that returns an instance of this contract after sending the transaction
        ///
        /// Notes:
        /// - If there are no constructor arguments, you should pass `()` as the argument.
        /// - The default poll duration is 7 seconds.
        /// - The default number of confirmations is 1 block.
        ///
        ///
        /// # Example
        ///
        /// Generate contract bindings with `abigen!` and deploy a new contract instance.
        ///
        /// *Note*: this requires a `bytecode` and `abi` object in the `greeter.json` artifact.
        ///
        /// ```ignore
        /// # async fn deploy<M: ethers::providers::Middleware>(client: ::std::sync::Arc<M>) {
        ///     abigen!(Greeter, "../greeter.json");
        ///
        ///    let greeter_contract = Greeter::deploy(client, "Hello world!".to_string()).unwrap().send().await.unwrap();
        ///    let msg = greeter_contract.greet().call().await.unwrap();
        /// # }
        /// ```
        pub fn deploy<T: ::ethers::core::abi::Tokenize>(
            client: ::std::sync::Arc<M>,
            constructor_args: T,
        ) -> ::core::result::Result<
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
        ///Calls the contract's `IS_SCRIPT` (0xf8ccbf47) function
        pub fn is_script(&self) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([248, 204, 191, 71], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `run` (0xef6ac0f0) function
        pub fn run(
            &self,
            p0: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([239, 106, 192, 240], p0)
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `proxy` event
        pub fn proxy_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, ProxyFilter> {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, ProxyFilter> {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for DeployProxyV0<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(name = "proxy", abi = "proxy(address)")]
    pub struct ProxyFilter(pub ::ethers::core::types::Address);
    ///Container type for all input parameters for the `IS_SCRIPT` function with signature `IS_SCRIPT()` and selector `0xf8ccbf47`
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
    #[ethcall(name = "IS_SCRIPT", abi = "IS_SCRIPT()")]
    pub struct IsScriptCall;
    ///Container type for all input parameters for the `run` function with signature `run(bytes32)` and selector `0xef6ac0f0`
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
    #[ethcall(name = "run", abi = "run(bytes32)")]
    pub struct RunCall(pub [u8; 32]);
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum DeployProxyV0Calls {
        IsScript(IsScriptCall),
        Run(RunCall),
    }
    impl ::ethers::core::abi::AbiDecode for DeployProxyV0Calls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded)
                = <IsScriptCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::IsScript(decoded));
            }
            if let Ok(decoded)
                = <RunCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Run(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for DeployProxyV0Calls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::IsScript(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Run(element) => ::ethers::core::abi::AbiEncode::encode(element),
            }
        }
    }
    impl ::core::fmt::Display for DeployProxyV0Calls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::IsScript(element) => ::core::fmt::Display::fmt(element, f),
                Self::Run(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<IsScriptCall> for DeployProxyV0Calls {
        fn from(value: IsScriptCall) -> Self {
            Self::IsScript(value)
        }
    }
    impl ::core::convert::From<RunCall> for DeployProxyV0Calls {
        fn from(value: RunCall) -> Self {
            Self::Run(value)
        }
    }
    ///Container type for all return fields from the `IS_SCRIPT` function with signature `IS_SCRIPT()` and selector `0xf8ccbf47`
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
    pub struct IsScriptReturn(pub bool);
}
