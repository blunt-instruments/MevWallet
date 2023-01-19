pub use mev_wallet_v0::*;
#[allow(clippy::too_many_arguments, non_camel_case_types)]
pub mod mev_wallet_v0 {
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
    #[doc = "MevWalletV0 was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    # [rustfmt :: skip] const __ABI : & str = "[{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"constructor\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"ExactBaseFee\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}],\"type\":\"error\",\"name\":\"HighBaseFee\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}],\"type\":\"error\",\"name\":\"MissingNonce\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint64\",\"name\":\"\",\"type\":\"uint64\",\"components\":[]}],\"type\":\"error\",\"name\":\"NotBefore\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"PermanentlyInvalid\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}],\"type\":\"error\",\"name\":\"ProvideValue\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"bytes\",\"name\":\"\",\"type\":\"bytes\",\"components\":[]}],\"type\":\"error\",\"name\":\"Reverted\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}],\"type\":\"error\",\"name\":\"UsedNonce\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}],\"type\":\"error\",\"name\":\"WrongSigner\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"nonce\",\"type\":\"uint256\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"Executed\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[],\"stateMutability\":\"payable\",\"type\":\"fallback\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"TX_TYPEHASH\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"_DOMAIN_SEPARATOR\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"newOwner\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"initialize\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"to\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"data\",\"type\":\"bytes\",\"components\":[]},{\"internalType\":\"int256\",\"name\":\"value\",\"type\":\"int256\",\"components\":[]},{\"internalType\":\"bool\",\"name\":\"delegate\",\"type\":\"bool\",\"components\":[]},{\"internalType\":\"int256\",\"name\":\"tip\",\"type\":\"int256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"maxBaseFee\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"timing\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"n\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint8\",\"name\":\"v\",\"type\":\"uint8\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"r\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"s\",\"type\":\"bytes32\",\"components\":[]}],\"stateMutability\":\"payable\",\"type\":\"function\",\"name\":\"mevTx\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"nonce\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"owner\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"newOwner\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"transferOwnership\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"payable\",\"type\":\"receive\",\"outputs\":[]}]" ;
    #[doc = r" The parsed JSON-ABI of the contract."]
    pub static MEVWALLETV0_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> =
        ::ethers::contract::Lazy::new(|| {
            ::ethers::core::utils::__serde_json::from_str(__ABI).expect("invalid abi")
        });
    #[doc = r" Bytecode of the #name contract"]
    pub static MEVWALLETV0_BYTECODE: ::ethers::contract::Lazy<::ethers::core::types::Bytes> =
        ::ethers::contract::Lazy::new(|| {
            "0x6080806040523461002957600180546001600160a01b03191660ff179055610dc1908161002f8239f35b600080fdfe608080604052600436101561001a575b50361561001857005b005b600090813560e01c9081638da5cb5b14610b9e57508063ae604e4514610b45578063affed0e014610b09578063c4d66de814610963578063dc0c81b514610928578063f2fde38b1461086e5763f31986760361000f576101607ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc360112610493576100a3610bee565b906024359167ffffffffffffffff831161086a573660238401121561086a5782600401356100d081610c9a565b936100de6040519586610c59565b8185523660248383010111610862578184926024602093018388013785010152606435151592836064350361086657610104359060ff82168203610862576084359461012c60443587610d39565b923a4803610838575a968351602085012092604051937ffd4c9c9ceea85482c3671626f7b1c65bc2230325b86dbcb0f971327c3062c26a602086015273ffffffffffffffffffffffffffffffffffffffff881660408601526060850152604435608085015260a084015260c083015260a43560e083015260c43561010083015261012060e4358184015282528161014081011067ffffffffffffffff6101408401111761080b57610140820160408190528251602084012087547f19010000000000000000000000000000000000000000000000000000000000006101608601526101628501526101828401526042815267ffffffffffffffff6101c0840190811191111761080b5760806101c08360ff8994836020970160405261014083015161016084012084840152166101e082015261012435610200820152610144356102208201528380520160015afa156108005773ffffffffffffffffffffffffffffffffffffffff84511680156107185773ffffffffffffffffffffffffffffffffffffffff6001541681036107cf575073ffffffffffffffffffffffffffffffffffffffff8316156107185760a4351515806107c4575b6107925767ffffffffffffffff421667ffffffffffffffff60c435169067ffffffffffffffff60c43560401c16918015159081610788575b506107185781801515918261077e575b505061074d575083604435126107185760643580610742575b6107185783604435138061070c575b6106da576002548060e435106106a9578060e4351161067857508380917fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff94856002558160643560001461065e5750602082519201905af46103b4610d84565b905b156105cb5750600160e435018060e435116105715760025560e4357fbcf6a68a2f901be4a23a41b53acd7697893a7e34def4e28acba584da75283b678480a25a840393841161059e57833a02933a8504143a15171561059e57829361041a91610d39565b90828213156104a557506e8c43efc014746c230049e330039cb390813b156104a15782916024839260405194859384927f5992bfdd00000000000000000000000000000000000000000000000000000000845260048401525af180156104965761048357505080f35b61048c90610c16565b6104935780f35b80fd5b6040513d84823e3d90fd5b5050fd5b91819003917f80000000000000000000000000000000000000000000000000000000000000008214600116610571578205036105445781906e8c43efc014746c230049e330039cb3803b156104a1576024839260405194859384927ff8e5c10200000000000000000000000000000000000000000000000000000000845260048401525af1801561049657610538575080f35b61054190610c16565b80f35b6024827f4e487b710000000000000000000000000000000000000000000000000000000081526011600452fd5b6024847f4e487b710000000000000000000000000000000000000000000000000000000081526011600452fd5b6024837f4e487b710000000000000000000000000000000000000000000000000000000081526011600452fd5b839060405180927fa815992000000000000000000000000000000000000000000000000000000000825260206004830152825192836024840152815b848110610646575050601f837fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe092604480968601015201168101030190fd5b60208282018101516044888401015286945001610607565b805192506020019034905af1610672610d84565b906103b6565b602490604051907f299aa7310000000000000000000000000000000000000000000000000000000082526004820152fd5b602490604051907f6ac964b00000000000000000000000000000000000000000000000000000000082526004820152fd5b60246040517f738833870000000000000000000000000000000000000000000000000000000081526044356004820152fd5b50604435341415610354565b60046040517fa04d981f000000000000000000000000000000000000000000000000000000008152fd5b506044351515610345565b602490604051907f08567e550000000000000000000000000000000000000000000000000000000082526004820152fd5b109050813861032c565b905081113861031c565b60246040517f74878d5800000000000000000000000000000000000000000000000000000000815260a4356004820152fd5b5060a43548116102e4565b602490604051907f32c15fc20000000000000000000000000000000000000000000000000000000082526004820152fd5b6040513d85823e3d90fd5b6024867f4e487b710000000000000000000000000000000000000000000000000000000081526041600452fd5b60046040517f2daf442d000000000000000000000000000000000000000000000000000000008152fd5b8380fd5b8280fd5b5080fd5b50346104935760207ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc360112610493576108a6610bee565b6001549073ffffffffffffffffffffffffffffffffffffffff8083163314801561091f575b156108625781169182151580610915575b156108625761090d7fffffffffffffffffffffffff0000000000000000000000000000000000000000923b15610cd4565b161760015580f35b50308314156108dc565b503033146108cb565b503461049357807ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc3601126104935760209054604051908152f35b50346104935760207ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc3601126104935761099b610bee565b6001549073ffffffffffffffffffffffffffffffffffffffff80831661086257816109e87fffffffffffffffffffffffff0000000000000000000000000000000000000000933b15610cd4565b16911617600155604051604081019067ffffffffffffffff9181811083821117610adc577f31000000000000000000000000000000000000000000000000000000000000009160209160405260018152015260405160208101917f8b73c3c69bb8fe3d512ecc4cf759cc79239f7b179b0ffacaa9a75d522b39400f83527fb057abb08031679648f38e605817690c05a381e6ec987e5d8cc0600a4d2a578660408301527fc89efdaa54c0f20c7adf612882df0950f5a951637e0307cdcb4c672f298b8bc660608301524660808301523060a083015260a0825260c082019082821090821117610adc57604052519020815580f35b6024847f4e487b710000000000000000000000000000000000000000000000000000000081526041600452fd5b503461049357807ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc360112610493576020600254604051908152f35b503461049357807ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc3601126104935760206040517ffd4c9c9ceea85482c3671626f7b1c65bc2230325b86dbcb0f971327c3062c26a8152f35b90503461086a57817ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc36011261086a5760209073ffffffffffffffffffffffffffffffffffffffff600154168152f35b6004359073ffffffffffffffffffffffffffffffffffffffff82168203610c1157565b600080fd5b67ffffffffffffffff8111610c2a57604052565b7f4e487b7100000000000000000000000000000000000000000000000000000000600052604160045260246000fd5b90601f7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0910116810190811067ffffffffffffffff821117610c2a57604052565b67ffffffffffffffff8111610c2a57601f017fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe01660200190565b15610cdb57565b60646040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152601160248201527f4e6f20636f6e7472616374206f776e65720000000000000000000000000000006044820152fd5b91909160008382019384129112908015821691151617610d5557565b7f4e487b7100000000000000000000000000000000000000000000000000000000600052601160045260246000fd5b3d15610daf573d90610d9582610c9a565b91610da36040519384610c59565b82523d6000602084013e565b60609056fea164736f6c6343000811000a" . parse () . expect ("invalid bytecode")
        });
    pub struct MevWalletV0<M>(::ethers::contract::Contract<M>);
    impl<M> Clone for MevWalletV0<M> {
        fn clone(&self) -> Self {
            MevWalletV0(self.0.clone())
        }
    }
    impl<M> std::ops::Deref for MevWalletV0<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> std::fmt::Debug for MevWalletV0<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(MevWalletV0))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> MevWalletV0<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ::ethers::contract::Contract::new(address.into(), MEVWALLETV0_ABI.clone(), client)
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
                MEVWALLETV0_ABI.clone(),
                MEVWALLETV0_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        #[doc = "Calls the contract's `TX_TYPEHASH` (0xae604e45) function"]
        pub fn tx_typehash(&self) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([174, 96, 78, 69], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `_DOMAIN_SEPARATOR` (0xdc0c81b5) function"]
        pub fn domain_separator(&self) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([220, 12, 129, 181], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `initialize` (0xc4d66de8) function"]
        pub fn initialize(
            &self,
            new_owner: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([196, 214, 109, 232], new_owner)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `mevTx` (0xf3198676) function"]
        pub fn mev_tx(
            &self,
            to: ::ethers::core::types::Address,
            data: ::ethers::core::types::Bytes,
            value: ::ethers::core::types::I256,
            delegate: bool,
            tip: ::ethers::core::types::I256,
            max_base_fee: ::ethers::core::types::U256,
            timing: ::ethers::core::types::U256,
            n: ::ethers::core::types::U256,
            v: u8,
            r: [u8; 32],
            s: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [243, 25, 134, 118],
                    (
                        to,
                        data,
                        value,
                        delegate,
                        tip,
                        max_base_fee,
                        timing,
                        n,
                        v,
                        r,
                        s,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `nonce` (0xaffed0e0) function"]
        pub fn nonce(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([175, 254, 208, 224], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `owner` (0x8da5cb5b) function"]
        pub fn owner(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([141, 165, 203, 91], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `transferOwnership` (0xf2fde38b) function"]
        pub fn transfer_ownership(
            &self,
            new_owner: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([242, 253, 227, 139], new_owner)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Gets the contract's `Executed` event"]
        pub fn executed_filter(&self) -> ::ethers::contract::builders::Event<M, ExecutedFilter> {
            self.0.event()
        }
        #[doc = r" Returns an [`Event`](#ethers_contract::builders::Event) builder for all events of this contract"]
        pub fn events(&self) -> ::ethers::contract::builders::Event<M, ExecutedFilter> {
            self.0.event_with_filter(Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>> for MevWalletV0<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self(contract)
        }
    }
    #[doc = "Custom Error type `ExactBaseFee` with signature `ExactBaseFee()` and selector `0x2daf442d`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        :: ethers :: contract :: EthError,
        :: ethers :: contract :: EthDisplay,
    )]
    #[etherror(name = "ExactBaseFee", abi = "ExactBaseFee()")]
    pub struct ExactBaseFee;
    #[doc = "Custom Error type `HighBaseFee` with signature `HighBaseFee(uint256)` and selector `0x74878d58`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        :: ethers :: contract :: EthError,
        :: ethers :: contract :: EthDisplay,
    )]
    #[etherror(name = "HighBaseFee", abi = "HighBaseFee(uint256)")]
    pub struct HighBaseFee(pub ::ethers::core::types::U256);
    #[doc = "Custom Error type `MissingNonce` with signature `MissingNonce(uint256)` and selector `0x299aa731`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        :: ethers :: contract :: EthError,
        :: ethers :: contract :: EthDisplay,
    )]
    #[etherror(name = "MissingNonce", abi = "MissingNonce(uint256)")]
    pub struct MissingNonce(pub ::ethers::core::types::U256);
    #[doc = "Custom Error type `NotBefore` with signature `NotBefore(uint64)` and selector `0x08567e55`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        :: ethers :: contract :: EthError,
        :: ethers :: contract :: EthDisplay,
    )]
    #[etherror(name = "NotBefore", abi = "NotBefore(uint64)")]
    pub struct NotBefore(pub u64);
    #[doc = "Custom Error type `PermanentlyInvalid` with signature `PermanentlyInvalid()` and selector `0xa04d981f`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        :: ethers :: contract :: EthError,
        :: ethers :: contract :: EthDisplay,
    )]
    #[etherror(name = "PermanentlyInvalid", abi = "PermanentlyInvalid()")]
    pub struct PermanentlyInvalid;
    #[doc = "Custom Error type `ProvideValue` with signature `ProvideValue(uint256)` and selector `0x73883387`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        :: ethers :: contract :: EthError,
        :: ethers :: contract :: EthDisplay,
    )]
    #[etherror(name = "ProvideValue", abi = "ProvideValue(uint256)")]
    pub struct ProvideValue(pub ::ethers::core::types::U256);
    #[doc = "Custom Error type `Reverted` with signature `Reverted(bytes)` and selector `0xa8159920`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        :: ethers :: contract :: EthError,
        :: ethers :: contract :: EthDisplay,
    )]
    #[etherror(name = "Reverted", abi = "Reverted(bytes)")]
    pub struct Reverted(pub ::ethers::core::types::Bytes);
    #[doc = "Custom Error type `UsedNonce` with signature `UsedNonce(uint256)` and selector `0x6ac964b0`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        :: ethers :: contract :: EthError,
        :: ethers :: contract :: EthDisplay,
    )]
    #[etherror(name = "UsedNonce", abi = "UsedNonce(uint256)")]
    pub struct UsedNonce(pub ::ethers::core::types::U256);
    #[doc = "Custom Error type `WrongSigner` with signature `WrongSigner(address)` and selector `0x32c15fc2`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        :: ethers :: contract :: EthError,
        :: ethers :: contract :: EthDisplay,
    )]
    #[etherror(name = "WrongSigner", abi = "WrongSigner(address)")]
    pub struct WrongSigner(pub ::ethers::core::types::Address);
    #[derive(Debug, Clone, PartialEq, Eq, :: ethers :: contract :: EthAbiType)]
    pub enum MevWalletV0Errors {
        ExactBaseFee(ExactBaseFee),
        HighBaseFee(HighBaseFee),
        MissingNonce(MissingNonce),
        NotBefore(NotBefore),
        PermanentlyInvalid(PermanentlyInvalid),
        ProvideValue(ProvideValue),
        Reverted(Reverted),
        UsedNonce(UsedNonce),
        WrongSigner(WrongSigner),
    }
    impl ::ethers::core::abi::AbiDecode for MevWalletV0Errors {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::std::result::Result<Self, ::ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <ExactBaseFee as ::ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MevWalletV0Errors::ExactBaseFee(decoded));
            }
            if let Ok(decoded) =
                <HighBaseFee as ::ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MevWalletV0Errors::HighBaseFee(decoded));
            }
            if let Ok(decoded) =
                <MissingNonce as ::ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MevWalletV0Errors::MissingNonce(decoded));
            }
            if let Ok(decoded) =
                <NotBefore as ::ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MevWalletV0Errors::NotBefore(decoded));
            }
            if let Ok(decoded) =
                <PermanentlyInvalid as ::ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MevWalletV0Errors::PermanentlyInvalid(decoded));
            }
            if let Ok(decoded) =
                <ProvideValue as ::ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MevWalletV0Errors::ProvideValue(decoded));
            }
            if let Ok(decoded) = <Reverted as ::ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MevWalletV0Errors::Reverted(decoded));
            }
            if let Ok(decoded) =
                <UsedNonce as ::ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MevWalletV0Errors::UsedNonce(decoded));
            }
            if let Ok(decoded) =
                <WrongSigner as ::ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MevWalletV0Errors::WrongSigner(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for MevWalletV0Errors {
        fn encode(self) -> Vec<u8> {
            match self {
                MevWalletV0Errors::ExactBaseFee(element) => element.encode(),
                MevWalletV0Errors::HighBaseFee(element) => element.encode(),
                MevWalletV0Errors::MissingNonce(element) => element.encode(),
                MevWalletV0Errors::NotBefore(element) => element.encode(),
                MevWalletV0Errors::PermanentlyInvalid(element) => element.encode(),
                MevWalletV0Errors::ProvideValue(element) => element.encode(),
                MevWalletV0Errors::Reverted(element) => element.encode(),
                MevWalletV0Errors::UsedNonce(element) => element.encode(),
                MevWalletV0Errors::WrongSigner(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for MevWalletV0Errors {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                MevWalletV0Errors::ExactBaseFee(element) => element.fmt(f),
                MevWalletV0Errors::HighBaseFee(element) => element.fmt(f),
                MevWalletV0Errors::MissingNonce(element) => element.fmt(f),
                MevWalletV0Errors::NotBefore(element) => element.fmt(f),
                MevWalletV0Errors::PermanentlyInvalid(element) => element.fmt(f),
                MevWalletV0Errors::ProvideValue(element) => element.fmt(f),
                MevWalletV0Errors::Reverted(element) => element.fmt(f),
                MevWalletV0Errors::UsedNonce(element) => element.fmt(f),
                MevWalletV0Errors::WrongSigner(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<ExactBaseFee> for MevWalletV0Errors {
        fn from(var: ExactBaseFee) -> Self {
            MevWalletV0Errors::ExactBaseFee(var)
        }
    }
    impl ::std::convert::From<HighBaseFee> for MevWalletV0Errors {
        fn from(var: HighBaseFee) -> Self {
            MevWalletV0Errors::HighBaseFee(var)
        }
    }
    impl ::std::convert::From<MissingNonce> for MevWalletV0Errors {
        fn from(var: MissingNonce) -> Self {
            MevWalletV0Errors::MissingNonce(var)
        }
    }
    impl ::std::convert::From<NotBefore> for MevWalletV0Errors {
        fn from(var: NotBefore) -> Self {
            MevWalletV0Errors::NotBefore(var)
        }
    }
    impl ::std::convert::From<PermanentlyInvalid> for MevWalletV0Errors {
        fn from(var: PermanentlyInvalid) -> Self {
            MevWalletV0Errors::PermanentlyInvalid(var)
        }
    }
    impl ::std::convert::From<ProvideValue> for MevWalletV0Errors {
        fn from(var: ProvideValue) -> Self {
            MevWalletV0Errors::ProvideValue(var)
        }
    }
    impl ::std::convert::From<Reverted> for MevWalletV0Errors {
        fn from(var: Reverted) -> Self {
            MevWalletV0Errors::Reverted(var)
        }
    }
    impl ::std::convert::From<UsedNonce> for MevWalletV0Errors {
        fn from(var: UsedNonce) -> Self {
            MevWalletV0Errors::UsedNonce(var)
        }
    }
    impl ::std::convert::From<WrongSigner> for MevWalletV0Errors {
        fn from(var: WrongSigner) -> Self {
            MevWalletV0Errors::WrongSigner(var)
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
    #[ethevent(name = "Executed", abi = "Executed(uint256)")]
    pub struct ExecutedFilter {
        #[ethevent(indexed)]
        pub nonce: ::ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `TX_TYPEHASH` function with signature `TX_TYPEHASH()` and selector `0xae604e45`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        :: ethers :: contract :: EthCall,
        :: ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "TX_TYPEHASH", abi = "TX_TYPEHASH()")]
    pub struct TxTypehashCall;
    #[doc = "Container type for all input parameters for the `_DOMAIN_SEPARATOR` function with signature `_DOMAIN_SEPARATOR()` and selector `0xdc0c81b5`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        :: ethers :: contract :: EthCall,
        :: ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "_DOMAIN_SEPARATOR", abi = "_DOMAIN_SEPARATOR()")]
    pub struct DomainSeparatorCall;
    #[doc = "Container type for all input parameters for the `initialize` function with signature `initialize(address)` and selector `0xc4d66de8`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        :: ethers :: contract :: EthCall,
        :: ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "initialize", abi = "initialize(address)")]
    pub struct InitializeCall {
        pub new_owner: ::ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `mevTx` function with signature `mevTx(address,bytes,int256,bool,int256,uint256,uint256,uint256,uint8,bytes32,bytes32)` and selector `0xf3198676`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        :: ethers :: contract :: EthCall,
        :: ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(
        name = "mevTx",
        abi = "mevTx(address,bytes,int256,bool,int256,uint256,uint256,uint256,uint8,bytes32,bytes32)"
    )]
    pub struct MevTxCall {
        pub to: ::ethers::core::types::Address,
        pub data: ::ethers::core::types::Bytes,
        pub value: ::ethers::core::types::I256,
        pub delegate: bool,
        pub tip: ::ethers::core::types::I256,
        pub max_base_fee: ::ethers::core::types::U256,
        pub timing: ::ethers::core::types::U256,
        pub n: ::ethers::core::types::U256,
        pub v: u8,
        pub r: [u8; 32],
        pub s: [u8; 32],
    }
    #[doc = "Container type for all input parameters for the `nonce` function with signature `nonce()` and selector `0xaffed0e0`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        :: ethers :: contract :: EthCall,
        :: ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "nonce", abi = "nonce()")]
    pub struct NonceCall;
    #[doc = "Container type for all input parameters for the `owner` function with signature `owner()` and selector `0x8da5cb5b`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        :: ethers :: contract :: EthCall,
        :: ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "owner", abi = "owner()")]
    pub struct OwnerCall;
    #[doc = "Container type for all input parameters for the `transferOwnership` function with signature `transferOwnership(address)` and selector `0xf2fde38b`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        :: ethers :: contract :: EthCall,
        :: ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "transferOwnership", abi = "transferOwnership(address)")]
    pub struct TransferOwnershipCall {
        pub new_owner: ::ethers::core::types::Address,
    }
    #[derive(Debug, Clone, PartialEq, Eq, :: ethers :: contract :: EthAbiType)]
    pub enum MevWalletV0Calls {
        TxTypehash(TxTypehashCall),
        DomainSeparator(DomainSeparatorCall),
        Initialize(InitializeCall),
        MevTx(MevTxCall),
        Nonce(NonceCall),
        Owner(OwnerCall),
        TransferOwnership(TransferOwnershipCall),
    }
    impl ::ethers::core::abi::AbiDecode for MevWalletV0Calls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::std::result::Result<Self, ::ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <TxTypehashCall as ::ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MevWalletV0Calls::TxTypehash(decoded));
            }
            if let Ok(decoded) =
                <DomainSeparatorCall as ::ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MevWalletV0Calls::DomainSeparator(decoded));
            }
            if let Ok(decoded) =
                <InitializeCall as ::ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MevWalletV0Calls::Initialize(decoded));
            }
            if let Ok(decoded) =
                <MevTxCall as ::ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MevWalletV0Calls::MevTx(decoded));
            }
            if let Ok(decoded) =
                <NonceCall as ::ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MevWalletV0Calls::Nonce(decoded));
            }
            if let Ok(decoded) =
                <OwnerCall as ::ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MevWalletV0Calls::Owner(decoded));
            }
            if let Ok(decoded) =
                <TransferOwnershipCall as ::ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MevWalletV0Calls::TransferOwnership(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for MevWalletV0Calls {
        fn encode(self) -> Vec<u8> {
            match self {
                MevWalletV0Calls::TxTypehash(element) => element.encode(),
                MevWalletV0Calls::DomainSeparator(element) => element.encode(),
                MevWalletV0Calls::Initialize(element) => element.encode(),
                MevWalletV0Calls::MevTx(element) => element.encode(),
                MevWalletV0Calls::Nonce(element) => element.encode(),
                MevWalletV0Calls::Owner(element) => element.encode(),
                MevWalletV0Calls::TransferOwnership(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for MevWalletV0Calls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                MevWalletV0Calls::TxTypehash(element) => element.fmt(f),
                MevWalletV0Calls::DomainSeparator(element) => element.fmt(f),
                MevWalletV0Calls::Initialize(element) => element.fmt(f),
                MevWalletV0Calls::MevTx(element) => element.fmt(f),
                MevWalletV0Calls::Nonce(element) => element.fmt(f),
                MevWalletV0Calls::Owner(element) => element.fmt(f),
                MevWalletV0Calls::TransferOwnership(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<TxTypehashCall> for MevWalletV0Calls {
        fn from(var: TxTypehashCall) -> Self {
            MevWalletV0Calls::TxTypehash(var)
        }
    }
    impl ::std::convert::From<DomainSeparatorCall> for MevWalletV0Calls {
        fn from(var: DomainSeparatorCall) -> Self {
            MevWalletV0Calls::DomainSeparator(var)
        }
    }
    impl ::std::convert::From<InitializeCall> for MevWalletV0Calls {
        fn from(var: InitializeCall) -> Self {
            MevWalletV0Calls::Initialize(var)
        }
    }
    impl ::std::convert::From<MevTxCall> for MevWalletV0Calls {
        fn from(var: MevTxCall) -> Self {
            MevWalletV0Calls::MevTx(var)
        }
    }
    impl ::std::convert::From<NonceCall> for MevWalletV0Calls {
        fn from(var: NonceCall) -> Self {
            MevWalletV0Calls::Nonce(var)
        }
    }
    impl ::std::convert::From<OwnerCall> for MevWalletV0Calls {
        fn from(var: OwnerCall) -> Self {
            MevWalletV0Calls::Owner(var)
        }
    }
    impl ::std::convert::From<TransferOwnershipCall> for MevWalletV0Calls {
        fn from(var: TransferOwnershipCall) -> Self {
            MevWalletV0Calls::TransferOwnership(var)
        }
    }
    #[doc = "Container type for all return fields from the `TX_TYPEHASH` function with signature `TX_TYPEHASH()` and selector `0xae604e45`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        :: ethers :: contract :: EthAbiType,
        :: ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct TxTypehashReturn(pub [u8; 32]);
    #[doc = "Container type for all return fields from the `_DOMAIN_SEPARATOR` function with signature `_DOMAIN_SEPARATOR()` and selector `0xdc0c81b5`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        :: ethers :: contract :: EthAbiType,
        :: ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct DomainSeparatorReturn(pub [u8; 32]);
    #[doc = "Container type for all return fields from the `nonce` function with signature `nonce()` and selector `0xaffed0e0`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        :: ethers :: contract :: EthAbiType,
        :: ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct NonceReturn(pub ::ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `owner` function with signature `owner()` and selector `0x8da5cb5b`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        :: ethers :: contract :: EthAbiType,
        :: ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct OwnerReturn(pub ::ethers::core::types::Address);
}
