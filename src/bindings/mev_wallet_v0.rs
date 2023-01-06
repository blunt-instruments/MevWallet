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
    # [rustfmt :: skip] const __ABI : & str = "[{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"constructor\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"ExactBaseFee\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}],\"type\":\"error\",\"name\":\"HighBaseFee\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}],\"type\":\"error\",\"name\":\"MissingNonce\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint64\",\"name\":\"\",\"type\":\"uint64\",\"components\":[]}],\"type\":\"error\",\"name\":\"NotBefore\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"PermanentlyInvalid\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}],\"type\":\"error\",\"name\":\"ProvideValue\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"bytes\",\"name\":\"\",\"type\":\"bytes\",\"components\":[]}],\"type\":\"error\",\"name\":\"Reverted\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}],\"type\":\"error\",\"name\":\"UsedNonce\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}],\"type\":\"error\",\"name\":\"WrongSigner\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"nonce\",\"type\":\"uint256\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"Executed\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[],\"stateMutability\":\"payable\",\"type\":\"fallback\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"TX_TYPEHASH\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"_DOMAIN_SEPARATOR\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_owner\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"initialize\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"to\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"data\",\"type\":\"bytes\",\"components\":[]},{\"internalType\":\"int256\",\"name\":\"value\",\"type\":\"int256\",\"components\":[]},{\"internalType\":\"bool\",\"name\":\"delegate\",\"type\":\"bool\",\"components\":[]},{\"internalType\":\"int256\",\"name\":\"tip\",\"type\":\"int256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"maxBaseFee\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"timing\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"n\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint8\",\"name\":\"v\",\"type\":\"uint8\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"r\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"s\",\"type\":\"bytes32\",\"components\":[]}],\"stateMutability\":\"payable\",\"type\":\"function\",\"name\":\"mevTx\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"nonce\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"owner\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"newOwner\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"transferOwnership\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"payable\",\"type\":\"receive\",\"outputs\":[]}]" ;
    #[doc = r" The parsed JSON-ABI of the contract."]
    pub static MEVWALLETV0_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            ethers::core::utils::__serde_json::from_str(__ABI).expect("invalid abi")
        });
    #[doc = r" Bytecode of the #name contract"]
    pub static MEVWALLETV0_BYTECODE: ethers::contract::Lazy<ethers::core::types::Bytes> =
        ethers::contract::Lazy::new(|| {
            "0x60a03461010d57600080546001600160a01b03191660ff1781556001600160401b039160408101838111828210176100f95760019160209160405282815201603160f81b815220906040519060208201927f8b73c3c69bb8fe3d512ecc4cf759cc79239f7b179b0ffacaa9a75d522b39400f84527fb057abb08031679648f38e605817690c05a381e6ec987e5d8cc0600a4d2a5786604084015260608301524660808301523060a083015260a0825260c0820193828510908511176100e5575082604052519020608052610931908161011382396080518181816101bd01526106f00152f35b634e487b7160e01b81526041600452602490fd5b634e487b7160e01b83526041600452602483fd5b600080fdfe6080806040526004908136101561001d575b5050361561001b57005b005b600090813560e01c9081638da5cb5b146107b257508063ae604e4514610777578063affed0e014610759578063c4d66de814610713578063dc0c81b5146106d8578063f2fde38b1461062a5763f319867603610011576101603660031901126103eb576100886107d6565b6024359067ffffffffffffffff821161046a573660238301121561046a57818401356100b38161083d565b926100c1604051948561081b565b8184523660248383010111610622578185926024602093018387013784010152606435151591826064350361062657610104359160ff83168303610622576084359261010f60443585610859565b933a4803610611575a956101a8604051809260208201947f5679fb6ec38d3c67731b4def49181a8fbbb334cda5c263b0993e50cfe699d4e8865260018060a01b0388166040840152610120606084015261016d61014084018a61088b565b91604435608085015260a084015260c083015260a43560e083015260c43561010083015260e43561012083015203601f19810183528261081b565b519020906040519161190160f01b60208401527f0000000000000000000000000000000000000000000000000000000000000000602284015260428301526042825281608081011067ffffffffffffffff6080840111176105fe576080808360ff8a948360209701604052825187840120848401521660a08201526101243560c08201526101443560e08201528380520160015afa156105f35784516001600160a01b031680156105e25785546001600160a01b031681036105cb57506001600160a01b0381161561054f5760a4351515806105c0575b6105a75767ffffffffffffffff421667ffffffffffffffff60c435169067ffffffffffffffff60c43560401c1691801515908161059d575b5061058c57818015159182610582575b505061056b5750846044351261054f5760643580610560575b61054f57846044351380610543575b61052a576001548060e43510610513578060e435116104fc57508491829181606435156104e25750602082519201905af46103286108cb565b905b156104ba575060e4357fbcf6a68a2f901be4a23a41b53acd7697893a7e34def4e28acba584da75283b678480a2600160e435018060e435116104a7576001555a820391821161049457813a02913a8304143a1517156104945782939161038f91610859565b90828213156103fe576e8c43efc014746c230049e330039cb391823b156103f95760248492836040519586948593635992bfdd60e01b85528401525af180156103ee576103db57505080f35b6103e4906107f1565b6103eb5780f35b80fd5b6040513d84823e3d90fd5b505050fd5b9181900390600160ff1b81146001166104815760001982050361046e5782906e8c43efc014746c230049e330039cb392833b1561046a57602490836040519586948593637c72e08160e11b85528401525af180156103ee5761045e575080f35b610467906107f1565b80f35b8280fd5b634e487b7160e01b835260118252602483fd5b634e487b7160e01b845260118352602484fd5b634e487b7160e01b835260118452602483fd5b634e487b7160e01b845260118552602484fd5b604051630540acc960e51b81526020818701529081906104de90602483019061088b565b0390fd5b805192506020019034905af16104f66108cb565b9061032a565b866024916040519163299aa73160e01b8352820152fd5b86602491604051916306ac964b60e41b8352820152fd5b604051637388338760e01b815260443581880152602490fd5b506044353414156102ef565b60405163a04d981f60e01b81528690fd5b5060443515156102e0565b86602491604051916308567e5560e01b8352820152fd5b10905081386102c7565b60405163a04d981f60e01b81528890fd5b90508111386102b7565b604051630e90f1ab60e31b815260a43581880152602490fd5b5060a435481161027f565b8660249160405191631960afe160e11b8352820152fd5b60405163a04d981f60e01b81528790fd5b6040513d86823e3d90fd5b634e487b7160e01b875260418852602487fd5b604051632daf442d60e01b81528890fd5b8480fd5b8380fd5b5090346106d45760203660031901126106d4576106456107d6565b825490916001600160a01b03338184161480156106cb575b1561062257831692831515806106c1575b15610622573b61068957506001600160a01b03191617815580f35b60649060206040519162461bcd60e51b8352820152601160248201527027379031b7b73a3930b1ba1037bbb732b960791b6044820152fd5b503084141561066e565b5030331461065d565b5080fd5b50346103eb57806003193601126103eb5760206040517f00000000000000000000000000000000000000000000000000000000000000008152f35b50346103eb5760203660031901126103eb5761072d6107d6565b8154906001600160a01b03908183166106265716906bffffffffffffffffffffffff60a01b1617815580f35b50346103eb57806003193601126103eb576020600154604051908152f35b50346103eb57806003193601126103eb5760206040517f5679fb6ec38d3c67731b4def49181a8fbbb334cda5c263b0993e50cfe699d4e88152f35b9050346106d457816003193601126106d45790546001600160a01b03168152602090f35b600435906001600160a01b03821682036107ec57565b600080fd5b67ffffffffffffffff811161080557604052565b634e487b7160e01b600052604160045260246000fd5b90601f8019910116810190811067ffffffffffffffff82111761080557604052565b67ffffffffffffffff811161080557601f01601f191660200190565b9190916000838201938412911290801582169115161761087557565b634e487b7160e01b600052601160045260246000fd5b919082519283825260005b8481106108b7575050826000602080949584010152601f8019910116010190565b602081830181015184830182015201610896565b3d156108f6573d906108dc8261083d565b916108ea604051938461081b565b82523d6000602084013e565b60609056fea264697066735822122010d53077ca90b1379c011f3e1f8ff552d48f65492269ed9aa9a24e1a807fdc3464736f6c63430008110033" . parse () . expect ("invalid bytecode")
        });
    pub struct MevWalletV0<M>(ethers::contract::Contract<M>);
    impl<M> Clone for MevWalletV0<M> {
        fn clone(&self) -> Self {
            MevWalletV0(self.0.clone())
        }
    }
    impl<M> std::ops::Deref for MevWalletV0<M> {
        type Target = ethers::contract::Contract<M>;
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
    impl<M: ethers::providers::Middleware> MevWalletV0<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(address.into(), MEVWALLETV0_ABI.clone(), client).into()
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
                MEVWALLETV0_ABI.clone(),
                MEVWALLETV0_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        #[doc = "Calls the contract's `TX_TYPEHASH` (0xae604e45) function"]
        pub fn tx_typehash(&self) -> ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([174, 96, 78, 69], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `_DOMAIN_SEPARATOR` (0xdc0c81b5) function"]
        pub fn domain_separator(&self) -> ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([220, 12, 129, 181], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `initialize` (0xc4d66de8) function"]
        pub fn initialize(
            &self,
            owner: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([196, 214, 109, 232], owner)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `mevTx` (0xf3198676) function"]
        pub fn mev_tx(
            &self,
            to: ethers::core::types::Address,
            data: ethers::core::types::Bytes,
            value: I256,
            delegate: bool,
            tip: I256,
            max_base_fee: ethers::core::types::U256,
            timing: ethers::core::types::U256,
            n: ethers::core::types::U256,
            v: u8,
            r: [u8; 32],
            s: [u8; 32],
        ) -> ethers::contract::builders::ContractCall<M, ()> {
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
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([175, 254, 208, 224], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `owner` (0x8da5cb5b) function"]
        pub fn owner(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([141, 165, 203, 91], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `transferOwnership` (0xf2fde38b) function"]
        pub fn transfer_ownership(
            &self,
            new_owner: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([242, 253, 227, 139], new_owner)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Gets the contract's `Executed` event"]
        pub fn executed_filter(&self) -> ethers::contract::builders::Event<M, ExecutedFilter> {
            self.0.event()
        }
        #[doc = r" Returns an [`Event`](#ethers_contract::builders::Event) builder for all events of this contract"]
        pub fn events(&self) -> ethers::contract::builders::Event<M, ExecutedFilter> {
            self.0.event_with_filter(Default::default())
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>> for MevWalletV0<M> {
        fn from(contract: ethers::contract::Contract<M>) -> Self {
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
        ethers :: contract :: EthError,
        ethers :: contract :: EthDisplay,
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
        ethers :: contract :: EthError,
        ethers :: contract :: EthDisplay,
    )]
    #[etherror(name = "HighBaseFee", abi = "HighBaseFee(uint256)")]
    pub struct HighBaseFee(pub ethers::core::types::U256);
    #[doc = "Custom Error type `MissingNonce` with signature `MissingNonce(uint256)` and selector `0x299aa731`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthError,
        ethers :: contract :: EthDisplay,
    )]
    #[etherror(name = "MissingNonce", abi = "MissingNonce(uint256)")]
    pub struct MissingNonce(pub ethers::core::types::U256);
    #[doc = "Custom Error type `NotBefore` with signature `NotBefore(uint64)` and selector `0x08567e55`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthError,
        ethers :: contract :: EthDisplay,
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
        ethers :: contract :: EthError,
        ethers :: contract :: EthDisplay,
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
        ethers :: contract :: EthError,
        ethers :: contract :: EthDisplay,
    )]
    #[etherror(name = "ProvideValue", abi = "ProvideValue(uint256)")]
    pub struct ProvideValue(pub ethers::core::types::U256);
    #[doc = "Custom Error type `Reverted` with signature `Reverted(bytes)` and selector `0xa8159920`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthError,
        ethers :: contract :: EthDisplay,
    )]
    #[etherror(name = "Reverted", abi = "Reverted(bytes)")]
    pub struct Reverted(pub ethers::core::types::Bytes);
    #[doc = "Custom Error type `UsedNonce` with signature `UsedNonce(uint256)` and selector `0x6ac964b0`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthError,
        ethers :: contract :: EthDisplay,
    )]
    #[etherror(name = "UsedNonce", abi = "UsedNonce(uint256)")]
    pub struct UsedNonce(pub ethers::core::types::U256);
    #[doc = "Custom Error type `WrongSigner` with signature `WrongSigner(address)` and selector `0x32c15fc2`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthError,
        ethers :: contract :: EthDisplay,
    )]
    #[etherror(name = "WrongSigner", abi = "WrongSigner(address)")]
    pub struct WrongSigner(pub ethers::core::types::Address);
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
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
    impl ethers::core::abi::AbiDecode for MevWalletV0Errors {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::std::result::Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <ExactBaseFee as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MevWalletV0Errors::ExactBaseFee(decoded));
            }
            if let Ok(decoded) =
                <HighBaseFee as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MevWalletV0Errors::HighBaseFee(decoded));
            }
            if let Ok(decoded) =
                <MissingNonce as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MevWalletV0Errors::MissingNonce(decoded));
            }
            if let Ok(decoded) = <NotBefore as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MevWalletV0Errors::NotBefore(decoded));
            }
            if let Ok(decoded) =
                <PermanentlyInvalid as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MevWalletV0Errors::PermanentlyInvalid(decoded));
            }
            if let Ok(decoded) =
                <ProvideValue as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MevWalletV0Errors::ProvideValue(decoded));
            }
            if let Ok(decoded) = <Reverted as ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(MevWalletV0Errors::Reverted(decoded));
            }
            if let Ok(decoded) = <UsedNonce as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MevWalletV0Errors::UsedNonce(decoded));
            }
            if let Ok(decoded) =
                <WrongSigner as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MevWalletV0Errors::WrongSigner(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for MevWalletV0Errors {
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
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethevent(name = "Executed", abi = "Executed(uint256)")]
    pub struct ExecutedFilter {
        #[ethevent(indexed)]
        pub nonce: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `TX_TYPEHASH` function with signature `TX_TYPEHASH()` and selector `0xae604e45`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
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
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
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
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "initialize", abi = "initialize(address)")]
    pub struct InitializeCall {
        pub owner: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `mevTx` function with signature `mevTx(address,bytes,int256,bool,int256,uint256,uint256,uint256,uint8,bytes32,bytes32)` and selector `0xf3198676`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(
        name = "mevTx",
        abi = "mevTx(address,bytes,int256,bool,int256,uint256,uint256,uint256,uint8,bytes32,bytes32)"
    )]
    pub struct MevTxCall {
        pub to: ethers::core::types::Address,
        pub data: ethers::core::types::Bytes,
        pub value: I256,
        pub delegate: bool,
        pub tip: I256,
        pub max_base_fee: ethers::core::types::U256,
        pub timing: ethers::core::types::U256,
        pub n: ethers::core::types::U256,
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
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
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
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
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
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "transferOwnership", abi = "transferOwnership(address)")]
    pub struct TransferOwnershipCall {
        pub new_owner: ethers::core::types::Address,
    }
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum MevWalletV0Calls {
        TxTypehash(TxTypehashCall),
        DomainSeparator(DomainSeparatorCall),
        Initialize(InitializeCall),
        MevTx(MevTxCall),
        Nonce(NonceCall),
        Owner(OwnerCall),
        TransferOwnership(TransferOwnershipCall),
    }
    impl ethers::core::abi::AbiDecode for MevWalletV0Calls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::std::result::Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <TxTypehashCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MevWalletV0Calls::TxTypehash(decoded));
            }
            if let Ok(decoded) =
                <DomainSeparatorCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MevWalletV0Calls::DomainSeparator(decoded));
            }
            if let Ok(decoded) =
                <InitializeCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MevWalletV0Calls::Initialize(decoded));
            }
            if let Ok(decoded) = <MevTxCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MevWalletV0Calls::MevTx(decoded));
            }
            if let Ok(decoded) = <NonceCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MevWalletV0Calls::Nonce(decoded));
            }
            if let Ok(decoded) = <OwnerCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MevWalletV0Calls::Owner(decoded));
            }
            if let Ok(decoded) =
                <TransferOwnershipCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MevWalletV0Calls::TransferOwnership(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for MevWalletV0Calls {
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
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct TxTypehashReturn(pub [u8; 32]);
    #[doc = "Container type for all return fields from the `_DOMAIN_SEPARATOR` function with signature `_DOMAIN_SEPARATOR()` and selector `0xdc0c81b5`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct DomainSeparatorReturn(pub [u8; 32]);
    #[doc = "Container type for all return fields from the `nonce` function with signature `nonce()` and selector `0xaffed0e0`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct NonceReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `owner` function with signature `owner()` and selector `0x8da5cb5b`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct OwnerReturn(pub ethers::core::types::Address);
}
