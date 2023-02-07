pub use deploy_drain::*;
#[allow(clippy::too_many_arguments, non_camel_case_types)]
pub mod deploy_drain {
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
    #[doc = "DeployDrain was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    # [rustfmt :: skip] const __ABI : & str = "[{\"inputs\":[{\"internalType\":\"address\",\"name\":\"to\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"bytes\",\"name\":\"data\",\"type\":\"bytes\",\"components\":[],\"indexed\":false},{\"internalType\":\"int256\",\"name\":\"value\",\"type\":\"int256\",\"components\":[],\"indexed\":false},{\"internalType\":\"bool\",\"name\":\"delegate\",\"type\":\"bool\",\"components\":[],\"indexed\":false},{\"internalType\":\"int256\",\"name\":\"tip\",\"type\":\"int256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"maxBaseFee\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"timing\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"nonce\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"MevTx\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"h\",\"type\":\"bytes32\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"ToSign\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"IS_SCRIPT\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"contract MevWalletV1\",\"name\":\"mevWallet\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"int256\",\"name\":\"tip\",\"type\":\"int256\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"to\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"drain\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"run\",\"outputs\":[]}]" ;
    #[doc = r" The parsed JSON-ABI of the contract."]
    pub static DEPLOYDRAIN_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> =
        ::ethers::contract::Lazy::new(|| {
            ::ethers::core::utils::__serde_json::from_str(__ABI).expect("invalid abi")
        });
    #[doc = r" Bytecode of the #name contract"]
    pub static DEPLOYDRAIN_BYTECODE: ::ethers::contract::Lazy<::ethers::core::types::Bytes> =
        ::ethers::contract::Lazy::new(|| {
            "0x6080806040523461002357600160ff19600c541617600c55610b0690816100298239f35b600080fdfe608080604052600436101561001357600080fd5b600090813560e01c9081636c79afb1146101ec57508063c0406226146100855763f8ccbf471461004257600080fd5b3461008257807ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc36011261008257602060ff600c54166040519015158152f35b80fd5b503461008257807ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc3601126100825780737109709ecfa91a80626ff3989d68f67f5b1dd12d803b156101e95781906004604051809481937fafc980400000000000000000000000000000000000000000000000000000000083525af180156101de57610199575b506040516103678082019082821067ffffffffffffffff83111761016c579180917fc5d2460186f7233c927e7db2dcc703c0e500b653ca82273b7bfad8045d85a470936107938339039083f5156101605780f35b604051903d90823e3d90fd5b6024847f4e487b710000000000000000000000000000000000000000000000000000000081526041600452fd5b67ffffffffffffffff81116101b1576040523861010c565b6024827f4e487b710000000000000000000000000000000000000000000000000000000081526041600452fd5b6040513d84823e3d90fd5b50fd5b90503461071e5760607ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc36011261071e5773ffffffffffffffffffffffffffffffffffffffff90600435828116810361071a57604435928084168403610703577fae604e450000000000000000000000000000000000000000000000000000000083526020836004818486165afa92831561070f5785936106d7575b506040519160208301937f101ae65e00000000000000000000000000000000000000000000000000000000855260048452604084019484861067ffffffffffffffff8711176105f6578560405284519020947faffed0e00000000000000000000000000000000000000000000000000000000081526020816004818787165afa9485156106cc57889561069b575b505060405194602086019182527325b6f6fdc8ee71b3571ddbac309c3c1194761d8d60408701526060860152866080860152600160a086015260243560c08601528660e08601528661010086015261012093848601528385528461014081011067ffffffffffffffff6101408701111761066e576101408501604081905285519091207fdc0c81b500000000000000000000000000000000000000000000000000000000825292906020906004818585165afa948515610663578795610623575b506040519260208401957f190100000000000000000000000000000000000000000000000000000000000087526022850152604284015260428352608083019483861067ffffffffffffffff8711176105f65781908660405284519020967f101ae65e0000000000000000000000000000000000000000000000000000000060a08601521660a48401526024855260e083019185831067ffffffffffffffff8411176105f6576004836020938193826040527faffed0e0000000000000000000000000000000000000000000000000000000008352165afa9081156105eb5786916105b5575b50604051937325b6f6fdc8ee71b3571ddbac309c3c1194761d8d85526101006020860152519182610100860152865b8381106105a057877f1cdf2e86ccc831ef96873febfee25ff7c41324e637caaec2f2265d032b1b2cc16020897f5679fb6ec38d3c67731b4def49181a8fbbb334cda5c263b0993e50cfe699d4e88a8a817fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0601f8d8d8b8683870101528b60408601526001606086015260243560808601528b60a08601528b60c086015260e085015201168101030190a1604051908152a180f35b81810160a001518682018601526020016104ec565b90506020823d6020116105e3575b816105d060209385610722565b810103126105df5751386104bd565b8580fd5b3d91506105c3565b6040513d88823e3d90fd5b6024887f4e487b710000000000000000000000000000000000000000000000000000000081526041600452fd5b9094506020813d60201161065b575b816106436020936101408401610722565b8101031261065757610140015193386103d7565b8680fd5b3d9150610632565b6040513d89823e3d90fd5b6024877f4e487b710000000000000000000000000000000000000000000000000000000081526041600452fd5b602080929396503d83116106c5575b6106b48185610722565b810103126106575751923880610316565b503d6106aa565b6040513d8a823e3d90fd5b9092506020813d602011610707575b816106f360209383610722565b8101031261070357519138610288565b8480fd5b3d91506106e6565b6040513d87823e3d90fd5b8380fd5b5080fd5b90601f7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0910116810190811067ffffffffffffffff82111761076357604052565b7f4e487b7100000000000000000000000000000000000000000000000000000000600052604160045260246000fdfe608080604052346100165761034b908161001c8239f35b600080fdfe6080604090808252600436101561001557600080fd5b600091823560e01c9182630a4432761461020257508163101ae65e1461008a575063ece531321461004557600080fd5b346100875760207ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc3601126100875761008461007f6102d0565b6102f8565b80f35b80fd5b9050346101bf5760207ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc3601126101bf576100c36102d0565b906100cd826102f8565b73ffffffffffffffffffffffffffffffffffffffff8151927f095ea7b30000000000000000000000000000000000000000000000000000000084521660048301527fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff6024830152602082604481866e8c43efc014746c230049e330039cb35af180156101f85761015b578280f35b6020903d82116101f0575b7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0601f83011683019083821067ffffffffffffffff8311176101c357526020908201829003126101bf5751801515036100875738808280f35b5080fd5b6024857f4e487b710000000000000000000000000000000000000000000000000000000081526041600452fd5b3d9150610166565b81513d85823e3d90fd5b9150346102cc57807ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc3601126102cc578261023b6102d0565b6024359073ffffffffffffffffffffffffffffffffffffffff918281168091036102c857602093869360449385937f095ea7b300000000000000000000000000000000000000000000000000000000855260048501527fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff6024850152165af180156101f85761015b578280f35b8380fd5b8280fd5b6004359073ffffffffffffffffffffffffffffffffffffffff821682036102f357565b600080fd5b600080808093479082908215610334575b73ffffffffffffffffffffffffffffffffffffffff1690f11561032857565b6040513d6000823e3d90fd5b6108fc915061030956fea164736f6c6343000811000aa164736f6c6343000811000a" . parse () . expect ("invalid bytecode")
        });
    pub struct DeployDrain<M>(::ethers::contract::Contract<M>);
    impl<M> Clone for DeployDrain<M> {
        fn clone(&self) -> Self {
            DeployDrain(self.0.clone())
        }
    }
    impl<M> std::ops::Deref for DeployDrain<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> std::fmt::Debug for DeployDrain<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(DeployDrain))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> DeployDrain<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ::ethers::contract::Contract::new(address.into(), DEPLOYDRAIN_ABI.clone(), client)
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
                DEPLOYDRAIN_ABI.clone(),
                DEPLOYDRAIN_BYTECODE.clone().into(),
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
        #[doc = "Calls the contract's `drain` (0x6c79afb1) function"]
        pub fn drain(
            &self,
            mev_wallet: ::ethers::core::types::Address,
            tip: ::ethers::core::types::I256,
            to: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([108, 121, 175, 177], (mev_wallet, tip, to))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `run` (0xc0406226) function"]
        pub fn run(&self) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([192, 64, 98, 38], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Gets the contract's `MevTx` event"]
        pub fn mev_tx_filter(&self) -> ::ethers::contract::builders::Event<M, MevTxFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `ToSign` event"]
        pub fn to_sign_filter(&self) -> ::ethers::contract::builders::Event<M, ToSignFilter> {
            self.0.event()
        }
        #[doc = r" Returns an [`Event`](#ethers_contract::builders::Event) builder for all events of this contract"]
        pub fn events(&self) -> ::ethers::contract::builders::Event<M, DeployDrainEvents> {
            self.0.event_with_filter(Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>> for DeployDrain<M> {
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
    #[ethevent(
        name = "MevTx",
        abi = "MevTx(address,bytes,int256,bool,int256,uint256,uint256,uint256)"
    )]
    pub struct MevTxFilter {
        pub to: ::ethers::core::types::Address,
        pub data: ::ethers::core::types::Bytes,
        pub value: ::ethers::core::types::I256,
        pub delegate: bool,
        pub tip: ::ethers::core::types::I256,
        pub max_base_fee: ::ethers::core::types::U256,
        pub timing: ::ethers::core::types::U256,
        pub nonce: ::ethers::core::types::U256,
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
    #[ethevent(name = "ToSign", abi = "ToSign(bytes32)")]
    pub struct ToSignFilter {
        pub h: [u8; 32],
    }
    #[derive(Debug, Clone, PartialEq, Eq, :: ethers :: contract :: EthAbiType)]
    pub enum DeployDrainEvents {
        MevTxFilter(MevTxFilter),
        ToSignFilter(ToSignFilter),
    }
    impl ::ethers::contract::EthLogDecode for DeployDrainEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::std::result::Result<Self, ::ethers::core::abi::Error>
        where
            Self: Sized,
        {
            if let Ok(decoded) = MevTxFilter::decode_log(log) {
                return Ok(DeployDrainEvents::MevTxFilter(decoded));
            }
            if let Ok(decoded) = ToSignFilter::decode_log(log) {
                return Ok(DeployDrainEvents::ToSignFilter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::std::fmt::Display for DeployDrainEvents {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                DeployDrainEvents::MevTxFilter(element) => element.fmt(f),
                DeployDrainEvents::ToSignFilter(element) => element.fmt(f),
            }
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
    #[doc = "Container type for all input parameters for the `drain` function with signature `drain(address,int256,address)` and selector `0x6c79afb1`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        :: ethers :: contract :: EthCall,
        :: ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "drain", abi = "drain(address,int256,address)")]
    pub struct DrainCall {
        pub mev_wallet: ::ethers::core::types::Address,
        pub tip: ::ethers::core::types::I256,
        pub to: ::ethers::core::types::Address,
    }
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
    #[derive(Debug, Clone, PartialEq, Eq, :: ethers :: contract :: EthAbiType)]
    pub enum DeployDrainCalls {
        IsScript(IsScriptCall),
        Drain(DrainCall),
        Run(RunCall),
    }
    impl ::ethers::core::abi::AbiDecode for DeployDrainCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::std::result::Result<Self, ::ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <IsScriptCall as ::ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(DeployDrainCalls::IsScript(decoded));
            }
            if let Ok(decoded) =
                <DrainCall as ::ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(DeployDrainCalls::Drain(decoded));
            }
            if let Ok(decoded) = <RunCall as ::ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(DeployDrainCalls::Run(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for DeployDrainCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                DeployDrainCalls::IsScript(element) => element.encode(),
                DeployDrainCalls::Drain(element) => element.encode(),
                DeployDrainCalls::Run(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for DeployDrainCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                DeployDrainCalls::IsScript(element) => element.fmt(f),
                DeployDrainCalls::Drain(element) => element.fmt(f),
                DeployDrainCalls::Run(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<IsScriptCall> for DeployDrainCalls {
        fn from(var: IsScriptCall) -> Self {
            DeployDrainCalls::IsScript(var)
        }
    }
    impl ::std::convert::From<DrainCall> for DeployDrainCalls {
        fn from(var: DrainCall) -> Self {
            DeployDrainCalls::Drain(var)
        }
    }
    impl ::std::convert::From<RunCall> for DeployDrainCalls {
        fn from(var: RunCall) -> Self {
            DeployDrainCalls::Run(var)
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
