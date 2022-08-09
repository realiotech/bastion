pub use land_bank::*;
#[allow(clippy::too_many_arguments, non_camel_case_types)]
pub mod land_bank {
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
    #[doc = "LandBank was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static LANDBANK_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            ethers :: core :: utils :: __serde_json :: from_str ("[{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_marketplace\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"_landNft\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"constructor\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_buyer\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"tokenId\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"buyLandFromBank\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"landNft\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"owner\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_seller\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"_tokenId\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"sellLandToBank\",\"outputs\":[]}]") . expect ("invalid abi")
        });
    #[doc = r" Bytecode of the #name contract"]
    pub static LANDBANK_BYTECODE: ethers::contract::Lazy<ethers::core::types::Bytes> =
        ethers::contract::Lazy::new(|| {
            "0x6080604052600160005534801561001557600080fd5b506040516106c43803806106c4833981016040819052610034916100f1565b6001600160a01b0382161580159061005457506001600160a01b03811615155b6100a45760405162461bcd60e51b815260206004820152601660248201527f63616e277420736574207a65726f206164647265737300000000000000000000604482015260640160405180910390fd5b600180546001600160a01b039384166001600160a01b03199182161790915560028054929093169116179055610124565b80516001600160a01b03811681146100ec57600080fd5b919050565b6000806040838503121561010457600080fd5b61010d836100d5565b915061011b602084016100d5565b90509250929050565b610591806101336000396000f3fe608060405234801561001057600080fd5b506004361061004c5760003560e01c8063036b68b7146100515780638da5cb5b14610066578063a1acbd7e14610095578063d33f1b1d146100a8575b600080fd5b61006461005f366004610421565b6100bb565b005b600154610079906001600160a01b031681565b6040516001600160a01b03909116815260200160405180910390f35b600254610079906001600160a01b031681565b6100646100b6366004610421565b6101a3565b6000546001146100ff5760405162461bcd60e51b815260206004820152600a6024820152695245454e5452414e435960b01b60448201526064015b60405180910390fd5b60026000556001546001600160a01b0316331461012e5760405162461bcd60e51b81526004016100f690610459565b6002546040516323b872dd60e01b81523060048201526001600160a01b03848116602483015260448201849052909116906323b872dd90606401600060405180830381600087803b15801561018257600080fd5b505af1158015610196573d6000803e3d6000fd5b5050600160005550505050565b6000546001146101e25760405162461bcd60e51b815260206004820152600a6024820152695245454e5452414e435960b01b60448201526064016100f6565b60026000556001546001600160a01b031633146102115760405162461bcd60e51b81526004016100f690610459565b600254604051630986e64760e01b81526004810183905273f21661d0d1d76d3ecb8e1b9f1c923dbfffae40979163a9059cbb9185916001600160a01b031690630986e64790602401602060405180830381865afa158015610276573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019061029a91906104a0565b600260009054906101000a90046001600160a01b03166001600160a01b031663618b2add6040518163ffffffff1660e01b81526004016020604051808303816000875af11580156102ef573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019061031391906104b9565b6040516370a0823160e01b815230600482015267ffffffffffffffff919091169073f21661d0d1d76d3ecb8e1b9f1c923dbfffae4097906370a0823190602401602060405180830381865afa158015610370573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019061039491906104a0565b61039e91906104ea565b6103a8919061050c565b6040516001600160e01b031960e085901b1681526001600160a01b03909216600483015260248201526044016020604051808303816000875af11580156103f3573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906104179190610539565b5050600160005550565b6000806040838503121561043457600080fd5b82356001600160a01b038116811461044b57600080fd5b946020939093013593505050565b60208082526027908201527f4f6e6c79206f776e657220636f6e74726163742063616e2072756e207472616e60408201526639b0b1ba34b7b760c91b606082015260800190565b6000602082840312156104b257600080fd5b5051919050565b6000602082840312156104cb57600080fd5b815167ffffffffffffffff811681146104e357600080fd5b9392505050565b60008261050757634e487b7160e01b600052601260045260246000fd5b500490565b600081600019048311821515161561053457634e487b7160e01b600052601160045260246000fd5b500290565b60006020828403121561054b57600080fd5b815180151581146104e357600080fdfea2646970667358221220c4691739c163c47cd6a4f8f70813f398291e7c68652cbfe6f806cd95e1626bef64736f6c634300080f0033" . parse () . expect ("invalid bytecode")
        });
    pub struct LandBank<M>(ethers::contract::Contract<M>);
    impl<M> Clone for LandBank<M> {
        fn clone(&self) -> Self {
            LandBank(self.0.clone())
        }
    }
    impl<M> std::ops::Deref for LandBank<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers::providers::Middleware> std::fmt::Debug for LandBank<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(LandBank))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ethers::providers::Middleware> LandBank<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(address.into(), LANDBANK_ABI.clone(), client).into()
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
                LANDBANK_ABI.clone(),
                LANDBANK_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        #[doc = "Calls the contract's `buyLandFromBank` (0x036b68b7) function"]
        pub fn buy_land_from_bank(
            &self,
            buyer: ethers::core::types::Address,
            token_id: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([3, 107, 104, 183], (buyer, token_id))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `landNft` (0xa1acbd7e) function"]
        pub fn land_nft(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([161, 172, 189, 126], ())
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
        #[doc = "Calls the contract's `sellLandToBank` (0xd33f1b1d) function"]
        pub fn sell_land_to_bank(
            &self,
            seller: ethers::core::types::Address,
            token_id: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([211, 63, 27, 29], (seller, token_id))
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>> for LandBank<M> {
        fn from(contract: ethers::contract::Contract<M>) -> Self {
            Self(contract)
        }
    }
    #[doc = "Container type for all input parameters for the `buyLandFromBank` function with signature `buyLandFromBank(address,uint256)` and selector `[3, 107, 104, 183]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "buyLandFromBank", abi = "buyLandFromBank(address,uint256)")]
    pub struct BuyLandFromBankCall {
        pub buyer: ethers::core::types::Address,
        pub token_id: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `landNft` function with signature `landNft()` and selector `[161, 172, 189, 126]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "landNft", abi = "landNft()")]
    pub struct LandNftCall;
    #[doc = "Container type for all input parameters for the `owner` function with signature `owner()` and selector `[141, 165, 203, 91]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "owner", abi = "owner()")]
    pub struct OwnerCall;
    #[doc = "Container type for all input parameters for the `sellLandToBank` function with signature `sellLandToBank(address,uint256)` and selector `[211, 63, 27, 29]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "sellLandToBank", abi = "sellLandToBank(address,uint256)")]
    pub struct SellLandToBankCall {
        pub seller: ethers::core::types::Address,
        pub token_id: ethers::core::types::U256,
    }
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum LandBankCalls {
        BuyLandFromBank(BuyLandFromBankCall),
        LandNft(LandNftCall),
        Owner(OwnerCall),
        SellLandToBank(SellLandToBankCall),
    }
    impl ethers::core::abi::AbiDecode for LandBankCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::std::result::Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <BuyLandFromBankCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(LandBankCalls::BuyLandFromBank(decoded));
            }
            if let Ok(decoded) =
                <LandNftCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(LandBankCalls::LandNft(decoded));
            }
            if let Ok(decoded) = <OwnerCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(LandBankCalls::Owner(decoded));
            }
            if let Ok(decoded) =
                <SellLandToBankCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(LandBankCalls::SellLandToBank(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for LandBankCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                LandBankCalls::BuyLandFromBank(element) => element.encode(),
                LandBankCalls::LandNft(element) => element.encode(),
                LandBankCalls::Owner(element) => element.encode(),
                LandBankCalls::SellLandToBank(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for LandBankCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                LandBankCalls::BuyLandFromBank(element) => element.fmt(f),
                LandBankCalls::LandNft(element) => element.fmt(f),
                LandBankCalls::Owner(element) => element.fmt(f),
                LandBankCalls::SellLandToBank(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<BuyLandFromBankCall> for LandBankCalls {
        fn from(var: BuyLandFromBankCall) -> Self {
            LandBankCalls::BuyLandFromBank(var)
        }
    }
    impl ::std::convert::From<LandNftCall> for LandBankCalls {
        fn from(var: LandNftCall) -> Self {
            LandBankCalls::LandNft(var)
        }
    }
    impl ::std::convert::From<OwnerCall> for LandBankCalls {
        fn from(var: OwnerCall) -> Self {
            LandBankCalls::Owner(var)
        }
    }
    impl ::std::convert::From<SellLandToBankCall> for LandBankCalls {
        fn from(var: SellLandToBankCall) -> Self {
            LandBankCalls::SellLandToBank(var)
        }
    }
    #[doc = "Container type for all return fields from the `landNft` function with signature `landNft()` and selector `[161, 172, 189, 126]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct LandNftReturn(pub ethers::core::types::Address);
    #[doc = "Container type for all return fields from the `owner` function with signature `owner()` and selector `[141, 165, 203, 91]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct OwnerReturn(pub ethers::core::types::Address);
}
