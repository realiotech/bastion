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
    # [rustfmt :: skip] const __ABI : & str = "[{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_devFund\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"_landNft\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"constructor\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"FailedTransfer\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"InsufficientEthBalance\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"InsufficientRio\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"coolDown\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_buyer\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256[]\",\"name\":\"landId\",\"type\":\"uint256[]\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"at\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"LandBought\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_seller\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256[]\",\"name\":\"landId\",\"type\":\"uint256[]\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"at\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"LandSold\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"WETH\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256[]\",\"name\":\"_tokenIds\",\"type\":\"uint256[]\",\"components\":[]}],\"stateMutability\":\"payable\",\"type\":\"function\",\"name\":\"buyLandFromBank\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"devFund\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_amountIn\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"address[]\",\"name\":\"path\",\"type\":\"address[]\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getAmountOutMin\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"getPrice\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"landNft\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"owner\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256[]\",\"name\":\"_tokenIds\",\"type\":\"uint256[]\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"sellLandToBank\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"payable\",\"type\":\"receive\",\"outputs\":[]}]" ;
    #[doc = r" The parsed JSON-ABI of the contract."]
    pub static LANDBANK_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            ethers::core::utils::__serde_json::from_str(__ABI).expect("invalid abi")
        });
    #[doc = r" Bytecode of the #name contract"]
    pub static LANDBANK_BYTECODE: ethers::contract::Lazy<ethers::core::types::Bytes> =
        ethers::contract::Lazy::new(|| {
            "0x6080604052600160005534801561001557600080fd5b50604051611349380380611349833981016040819052610034916100f1565b6001600160a01b0382161580159061005457506001600160a01b03811615155b6100a45760405162461bcd60e51b815260206004820152601660248201527f63616e277420736574207a65726f206164647265737300000000000000000000604482015260640160405180910390fd5b600380546001600160a01b039384166001600160a01b03199182161790915560028054929093169116179055610124565b80516001600160a01b03811681146100ec57600080fd5b919050565b6000806040838503121561010457600080fd5b61010d836100d5565b915061011b602084016100d5565b90509250929050565b611216806101336000396000f3fe60806040526004361061007f5760003560e01c806398d5fdca1161004e57806398d5fdca14610138578063a1acbd7e1461014d578063ad5c46481461016d578063df17dcd91461019557600080fd5b80631fffbe7f1461008b5780634390d2a8146100ad57806348334429146100ea5780638da5cb5b1461011857600080fd5b3661008657005b600080fd5b34801561009757600080fd5b506100ab6100a6366004610da4565b6101a8565b005b3480156100b957600080fd5b506003546100cd906001600160a01b031681565b6040516001600160a01b0390911681526020015b60405180910390f35b3480156100f657600080fd5b5061010a610105366004610e3a565b6103b5565b6040519081526020016100e1565b34801561012457600080fd5b506001546100cd906001600160a01b031681565b34801561014457600080fd5b5061010a61046a565b34801561015957600080fd5b506002546100cd906001600160a01b031681565b34801561017957600080fd5b506100cd73c02aaa39b223fe8d0a0e5c4f27ead9083c756cc281565b6100ab6101a3366004610da4565b610598565b6000546001146101ec5760405162461bcd60e51b815260206004820152600a6024820152695245454e5452414e435960b01b60448201526064015b60405180910390fd5b60026000908155815190816101ff61046a565b02905060005b828110156102f05760025484516001600160a01b03909116906323b872dd903390309088908690811061023a5761023a610eed565b60209081029190910101516040516001600160e01b031960e086901b1681526001600160a01b0393841660048201529290911660248301526044820152606401600060405180830381600087803b15801561029457600080fd5b505af11580156102a8573d6000803e3d6000fd5b5050505042600460008684815181106102c3576102c3610eed565b602002602001015181526020019081526020016000208190555080806102e890610f19565b915050610205565b5060405163a9059cbb60e01b81523360048201526024810182905273f21661d0d1d76d3ecb8e1b9f1c923dbfffae40979063a9059cbb906044016020604051808303816000875af1158015610349573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019061036d9190610f32565b507f8b01d96644460f984e6836351bf037940d5db9407477a090a8deabdf241b8495338483426040516103a39493929190610f5b565b60405180910390a15050600160005550565b60405163d06ca61f60e01b81526000908190737a250d5630b4cf539739df2c5dacb4c659f2488d9063d06ca61f906103f39087908790600401610fff565b600060405180830381865afa158015610410573d6000803e3d6000fd5b505050506040513d6000823e601f3d908101601f191682016040526104389190810190611020565b9050806001845161044991906110a6565b8151811061045957610459610eed565b602002602001015191505092915050565b6040516370a0823160e01b8152306004820152600090819073f21661d0d1d76d3ecb8e1b9f1c923dbfffae4097906370a0823190602401602060405180830381865afa1580156104be573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906104e291906110bd565b90506000600260009054906101000a90046001600160a01b03166001600160a01b031663618b2add6040518163ffffffff1660e01b81526004016020604051808303816000875af115801561053b573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019061055f91906110d6565b67ffffffffffffffff1690506000600a82848161057e5761057e611100565b04600c028161058f5761058f611100565b04949350505050565b6000546001146105d75760405162461bcd60e51b815260206004820152600a6024820152695245454e5452414e435960b01b60448201526064016101e3565b600260009081558151906105e961046a565b6105f39083611116565b905060005b835181101561066e57426004600086848151811061061857610618610eed565b60200260200101518152602001908152602001600020546206978061063d9190611135565b111561065c57604051634d056f6d60e01b815260040160405180910390fd5b8061066681610f19565b9150506105f8565b5060408051600280825260608201835260009260208301908036833701905050905073f21661d0d1d76d3ecb8e1b9f1c923dbfffae4097816000815181106106b8576106b8610eed565b60200260200101906001600160a01b031690816001600160a01b03168152505073c02aaa39b223fe8d0a0e5c4f27ead9083c756cc28160018151811061070057610700610eed565b60200260200101906001600160a01b031690816001600160a01b031681525050346000036109a2576040516370a0823160e01b8152336004820152829073f21661d0d1d76d3ecb8e1b9f1c923dbfffae4097906370a0823190602401602060405180830381865afa158015610779573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019061079d91906110bd565b10156107bc57604051630780674d60e11b815260040160405180910390fd5b6040516323b872dd60e01b81523360048201523060248201526044810183905260009073f21661d0d1d76d3ecb8e1b9f1c923dbfffae4097906323b872dd906064016020604051808303816000875af115801561081d573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906108419190610f32565b9050806108615760405163bfa871c560e01b815260040160405180910390fd5b6000606461087085600a611116565b61087a919061114d565b9050600061088882856103b5565b60405163095ea7b360e01b8152737a250d5630b4cf539739df2c5dacb4c659f2488d60048201526024810184905290915073f21661d0d1d76d3ecb8e1b9f1c923dbfffae40979063095ea7b3906044016020604051808303816000875af11580156108f7573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019061091b9190610f32565b506003546040516318cbafe560e01b8152737a250d5630b4cf539739df2c5dacb4c659f2488d916318cbafe59161096891869186918a916001600160a01b0390911690429060040161116f565b600060405180830381600087803b15801561098257600080fd5b505af1158015610996573d6000803e3d6000fd5b50505050505050610c02565b6000341180610a2157506040516370a0823160e01b815233600482015273f21661d0d1d76d3ecb8e1b9f1c923dbfffae4097906370a0823190602401602060405180830381865afa1580156109fb573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190610a1f91906110bd565b155b15610c02576000610a3283836103b5565b905080341015610a5557604051635b6b73eb60e11b815260040160405180910390fd5b6003546001600160a01b03166108fc610a6f600a3461114d565b6040518115909202916000818181858888f19350505050158015610a97573d6000803e3d6000fd5b506000600a610aa7346009611116565b610ab1919061114d565b604080516002808252606082018352929350600092909160208301908036833701905050905073c02aaa39b223fe8d0a0e5c4f27ead9083c756cc281600081518110610aff57610aff610eed565b60200260200101906001600160a01b031690816001600160a01b03168152505073f21661d0d1d76d3ecb8e1b9f1c923dbfffae409781600181518110610b4757610b47610eed565b60200260200101906001600160a01b031690816001600160a01b0316815250506000610b7383866103b5565b604051637ff36ab560e01b8152909150737a250d5630b4cf539739df2c5dacb4c659f2488d90637ff36ab5908590610bb59085908790309042906004016111ab565b60006040518083038185885af1158015610bd3573d6000803e3d6000fd5b50505050506040513d6000823e601f3d908101601f19168201604052610bfc9190810190611020565b50505050505b60005b83811015610cf05760025485516001600160a01b03909116906342842e0e9030903390899086908110610c3a57610c3a610eed565b60209081029190910101516040516001600160e01b031960e086901b1681526001600160a01b0393841660048201529290911660248301526044820152606401600060405180830381600087803b158015610c9457600080fd5b505af1158015610ca8573d6000803e3d6000fd5b505050504260046000878481518110610cc357610cc3610eed565b60200260200101518152602001908152602001600020819055508080610ce890610f19565b915050610c05565b507f63f6eae93c684489ef48a7c99798f6a2ffce3c3266b9a9e7c291930772f4eec433858442604051610d269493929190610f5b565b60405180910390a1505060016000555050565b634e487b7160e01b600052604160045260246000fd5b604051601f8201601f1916810167ffffffffffffffff81118282101715610d7857610d78610d39565b604052919050565b600067ffffffffffffffff821115610d9a57610d9a610d39565b5060051b60200190565b60006020808385031215610db757600080fd5b823567ffffffffffffffff811115610dce57600080fd5b8301601f81018513610ddf57600080fd5b8035610df2610ded82610d80565b610d4f565b81815260059190911b82018301908381019087831115610e1157600080fd5b928401925b82841015610e2f57833582529284019290840190610e16565b979650505050505050565b60008060408385031215610e4d57600080fd5b8235915060208084013567ffffffffffffffff811115610e6c57600080fd5b8401601f81018613610e7d57600080fd5b8035610e8b610ded82610d80565b81815260059190911b82018301908381019088831115610eaa57600080fd5b928401925b82841015610ede5783356001600160a01b0381168114610ecf5760008081fd5b82529284019290840190610eaf565b80955050505050509250929050565b634e487b7160e01b600052603260045260246000fd5b634e487b7160e01b600052601160045260246000fd5b600060018201610f2b57610f2b610f03565b5060010190565b600060208284031215610f4457600080fd5b81518015158114610f5457600080fd5b9392505050565b6001600160a01b0385168152608060208083018290528551918301829052600091868201919060a0850190845b81811015610fa457845183529383019391830191600101610f88565b505060408501969096525050506060015292915050565b600081518084526020808501945080840160005b83811015610ff45781516001600160a01b031687529582019590820190600101610fcf565b509495945050505050565b8281526040602082015260006110186040830184610fbb565b949350505050565b6000602080838503121561103357600080fd5b825167ffffffffffffffff81111561104a57600080fd5b8301601f8101851361105b57600080fd5b8051611069610ded82610d80565b81815260059190911b8201830190838101908783111561108857600080fd5b928401925b82841015610e2f5783518252928401929084019061108d565b6000828210156110b8576110b8610f03565b500390565b6000602082840312156110cf57600080fd5b5051919050565b6000602082840312156110e857600080fd5b815167ffffffffffffffff81168114610f5457600080fd5b634e487b7160e01b600052601260045260246000fd5b600081600019048311821515161561113057611130610f03565b500290565b6000821982111561114857611148610f03565b500190565b60008261116a57634e487b7160e01b600052601260045260246000fd5b500490565b85815284602082015260a06040820152600061118e60a0830186610fbb565b6001600160a01b0394909416606083015250608001529392505050565b8481526080602082015260006111c46080830186610fbb565b6001600160a01b0394909416604083015250606001529291505056fea26469706673582212207fc09be0744ea7d90754c357a7c9682d21b381c1f8622b496dd6f373d386988564736f6c634300080f0033" . parse () . expect ("invalid bytecode")
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
    impl<M> std::fmt::Debug for LandBank<M> {
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
        #[doc = "Calls the contract's `WETH` (0xad5c4648) function"]
        pub fn weth(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([173, 92, 70, 72], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `buyLandFromBank` (0xdf17dcd9) function"]
        pub fn buy_land_from_bank(
            &self,
            token_ids: ::std::vec::Vec<ethers::core::types::U256>,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([223, 23, 220, 217], token_ids)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `devFund` (0x4390d2a8) function"]
        pub fn dev_fund(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([67, 144, 210, 168], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getAmountOutMin` (0x48334429) function"]
        pub fn get_amount_out_min(
            &self,
            amount_in: ethers::core::types::U256,
            path: ::std::vec::Vec<ethers::core::types::Address>,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([72, 51, 68, 41], (amount_in, path))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getPrice` (0x98d5fdca) function"]
        pub fn get_price(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([152, 213, 253, 202], ())
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
        #[doc = "Calls the contract's `sellLandToBank` (0x1fffbe7f) function"]
        pub fn sell_land_to_bank(
            &self,
            token_ids: ::std::vec::Vec<ethers::core::types::U256>,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([31, 255, 190, 127], token_ids)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Gets the contract's `LandBought` event"]
        pub fn land_bought_filter(&self) -> ethers::contract::builders::Event<M, LandBoughtFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `LandSold` event"]
        pub fn land_sold_filter(&self) -> ethers::contract::builders::Event<M, LandSoldFilter> {
            self.0.event()
        }
        #[doc = r" Returns an [`Event`](#ethers_contract::builders::Event) builder for all events of this contract"]
        pub fn events(&self) -> ethers::contract::builders::Event<M, LandBankEvents> {
            self.0.event_with_filter(Default::default())
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>> for LandBank<M> {
        fn from(contract: ethers::contract::Contract<M>) -> Self {
            Self(contract)
        }
    }
    #[doc = "Custom Error type `FailedTransfer` with signature `FailedTransfer()` and selector `[191, 168, 113, 197]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthError,
        ethers :: contract :: EthDisplay,
    )]
    #[etherror(name = "FailedTransfer", abi = "FailedTransfer()")]
    pub struct FailedTransfer;
    #[doc = "Custom Error type `InsufficientEthBalance` with signature `InsufficientEthBalance()` and selector `[182, 214, 231, 214]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthError,
        ethers :: contract :: EthDisplay,
    )]
    #[etherror(name = "InsufficientEthBalance", abi = "InsufficientEthBalance()")]
    pub struct InsufficientEthBalance;
    #[doc = "Custom Error type `InsufficientRio` with signature `InsufficientRio()` and selector `[15, 0, 206, 154]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthError,
        ethers :: contract :: EthDisplay,
    )]
    #[etherror(name = "InsufficientRio", abi = "InsufficientRio()")]
    pub struct InsufficientRio;
    #[doc = "Custom Error type `coolDown` with signature `coolDown()` and selector `[77, 5, 111, 109]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthError,
        ethers :: contract :: EthDisplay,
    )]
    #[etherror(name = "coolDown", abi = "coolDown()")]
    pub struct coolDown;
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum LandBankErrors {
        FailedTransfer(FailedTransfer),
        InsufficientEthBalance(InsufficientEthBalance),
        InsufficientRio(InsufficientRio),
        coolDown(coolDown),
    }
    impl ethers::core::abi::AbiDecode for LandBankErrors {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::std::result::Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <FailedTransfer as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(LandBankErrors::FailedTransfer(decoded));
            }
            if let Ok(decoded) =
                <InsufficientEthBalance as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(LandBankErrors::InsufficientEthBalance(decoded));
            }
            if let Ok(decoded) =
                <InsufficientRio as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(LandBankErrors::InsufficientRio(decoded));
            }
            if let Ok(decoded) = <coolDown as ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(LandBankErrors::coolDown(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for LandBankErrors {
        fn encode(self) -> Vec<u8> {
            match self {
                LandBankErrors::FailedTransfer(element) => element.encode(),
                LandBankErrors::InsufficientEthBalance(element) => element.encode(),
                LandBankErrors::InsufficientRio(element) => element.encode(),
                LandBankErrors::coolDown(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for LandBankErrors {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                LandBankErrors::FailedTransfer(element) => element.fmt(f),
                LandBankErrors::InsufficientEthBalance(element) => element.fmt(f),
                LandBankErrors::InsufficientRio(element) => element.fmt(f),
                LandBankErrors::coolDown(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<FailedTransfer> for LandBankErrors {
        fn from(var: FailedTransfer) -> Self {
            LandBankErrors::FailedTransfer(var)
        }
    }
    impl ::std::convert::From<InsufficientEthBalance> for LandBankErrors {
        fn from(var: InsufficientEthBalance) -> Self {
            LandBankErrors::InsufficientEthBalance(var)
        }
    }
    impl ::std::convert::From<InsufficientRio> for LandBankErrors {
        fn from(var: InsufficientRio) -> Self {
            LandBankErrors::InsufficientRio(var)
        }
    }
    impl ::std::convert::From<coolDown> for LandBankErrors {
        fn from(var: coolDown) -> Self {
            LandBankErrors::coolDown(var)
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
    #[ethevent(
        name = "LandBought",
        abi = "LandBought(address,uint256[],uint256,uint256)"
    )]
    pub struct LandBoughtFilter {
        pub buyer: ethers::core::types::Address,
        pub land_id: Vec<ethers::core::types::U256>,
        pub amount: ethers::core::types::U256,
        pub at: ethers::core::types::U256,
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
    #[ethevent(name = "LandSold", abi = "LandSold(address,uint256[],uint256,uint256)")]
    pub struct LandSoldFilter {
        pub seller: ethers::core::types::Address,
        pub land_id: Vec<ethers::core::types::U256>,
        pub amount: ethers::core::types::U256,
        pub at: ethers::core::types::U256,
    }
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum LandBankEvents {
        LandBoughtFilter(LandBoughtFilter),
        LandSoldFilter(LandSoldFilter),
    }
    impl ethers::contract::EthLogDecode for LandBankEvents {
        fn decode_log(
            log: &ethers::core::abi::RawLog,
        ) -> ::std::result::Result<Self, ethers::core::abi::Error>
        where
            Self: Sized,
        {
            if let Ok(decoded) = LandBoughtFilter::decode_log(log) {
                return Ok(LandBankEvents::LandBoughtFilter(decoded));
            }
            if let Ok(decoded) = LandSoldFilter::decode_log(log) {
                return Ok(LandBankEvents::LandSoldFilter(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::std::fmt::Display for LandBankEvents {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                LandBankEvents::LandBoughtFilter(element) => element.fmt(f),
                LandBankEvents::LandSoldFilter(element) => element.fmt(f),
            }
        }
    }
    #[doc = "Container type for all input parameters for the `WETH` function with signature `WETH()` and selector `[173, 92, 70, 72]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "WETH", abi = "WETH()")]
    pub struct WethCall;
    #[doc = "Container type for all input parameters for the `buyLandFromBank` function with signature `buyLandFromBank(uint256[])` and selector `[223, 23, 220, 217]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "buyLandFromBank", abi = "buyLandFromBank(uint256[])")]
    pub struct BuyLandFromBankCall {
        pub token_ids: ::std::vec::Vec<ethers::core::types::U256>,
    }
    #[doc = "Container type for all input parameters for the `devFund` function with signature `devFund()` and selector `[67, 144, 210, 168]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "devFund", abi = "devFund()")]
    pub struct DevFundCall;
    #[doc = "Container type for all input parameters for the `getAmountOutMin` function with signature `getAmountOutMin(uint256,address[])` and selector `[72, 51, 68, 41]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "getAmountOutMin", abi = "getAmountOutMin(uint256,address[])")]
    pub struct GetAmountOutMinCall {
        pub amount_in: ethers::core::types::U256,
        pub path: ::std::vec::Vec<ethers::core::types::Address>,
    }
    #[doc = "Container type for all input parameters for the `getPrice` function with signature `getPrice()` and selector `[152, 213, 253, 202]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "getPrice", abi = "getPrice()")]
    pub struct GetPriceCall;
    #[doc = "Container type for all input parameters for the `landNft` function with signature `landNft()` and selector `[161, 172, 189, 126]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "landNft", abi = "landNft()")]
    pub struct LandNftCall;
    #[doc = "Container type for all input parameters for the `owner` function with signature `owner()` and selector `[141, 165, 203, 91]`"]
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
    #[doc = "Container type for all input parameters for the `sellLandToBank` function with signature `sellLandToBank(uint256[])` and selector `[31, 255, 190, 127]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "sellLandToBank", abi = "sellLandToBank(uint256[])")]
    pub struct SellLandToBankCall {
        pub token_ids: ::std::vec::Vec<ethers::core::types::U256>,
    }
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum LandBankCalls {
        Weth(WethCall),
        BuyLandFromBank(BuyLandFromBankCall),
        DevFund(DevFundCall),
        GetAmountOutMin(GetAmountOutMinCall),
        GetPrice(GetPriceCall),
        LandNft(LandNftCall),
        Owner(OwnerCall),
        SellLandToBank(SellLandToBankCall),
    }
    impl ethers::core::abi::AbiDecode for LandBankCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::std::result::Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) = <WethCall as ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(LandBankCalls::Weth(decoded));
            }
            if let Ok(decoded) =
                <BuyLandFromBankCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(LandBankCalls::BuyLandFromBank(decoded));
            }
            if let Ok(decoded) =
                <DevFundCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(LandBankCalls::DevFund(decoded));
            }
            if let Ok(decoded) =
                <GetAmountOutMinCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(LandBankCalls::GetAmountOutMin(decoded));
            }
            if let Ok(decoded) =
                <GetPriceCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(LandBankCalls::GetPrice(decoded));
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
                LandBankCalls::Weth(element) => element.encode(),
                LandBankCalls::BuyLandFromBank(element) => element.encode(),
                LandBankCalls::DevFund(element) => element.encode(),
                LandBankCalls::GetAmountOutMin(element) => element.encode(),
                LandBankCalls::GetPrice(element) => element.encode(),
                LandBankCalls::LandNft(element) => element.encode(),
                LandBankCalls::Owner(element) => element.encode(),
                LandBankCalls::SellLandToBank(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for LandBankCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                LandBankCalls::Weth(element) => element.fmt(f),
                LandBankCalls::BuyLandFromBank(element) => element.fmt(f),
                LandBankCalls::DevFund(element) => element.fmt(f),
                LandBankCalls::GetAmountOutMin(element) => element.fmt(f),
                LandBankCalls::GetPrice(element) => element.fmt(f),
                LandBankCalls::LandNft(element) => element.fmt(f),
                LandBankCalls::Owner(element) => element.fmt(f),
                LandBankCalls::SellLandToBank(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<WethCall> for LandBankCalls {
        fn from(var: WethCall) -> Self {
            LandBankCalls::Weth(var)
        }
    }
    impl ::std::convert::From<BuyLandFromBankCall> for LandBankCalls {
        fn from(var: BuyLandFromBankCall) -> Self {
            LandBankCalls::BuyLandFromBank(var)
        }
    }
    impl ::std::convert::From<DevFundCall> for LandBankCalls {
        fn from(var: DevFundCall) -> Self {
            LandBankCalls::DevFund(var)
        }
    }
    impl ::std::convert::From<GetAmountOutMinCall> for LandBankCalls {
        fn from(var: GetAmountOutMinCall) -> Self {
            LandBankCalls::GetAmountOutMin(var)
        }
    }
    impl ::std::convert::From<GetPriceCall> for LandBankCalls {
        fn from(var: GetPriceCall) -> Self {
            LandBankCalls::GetPrice(var)
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
    #[doc = "Container type for all return fields from the `WETH` function with signature `WETH()` and selector `[173, 92, 70, 72]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct WethReturn(pub ethers::core::types::Address);
    #[doc = "Container type for all return fields from the `devFund` function with signature `devFund()` and selector `[67, 144, 210, 168]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct DevFundReturn(pub ethers::core::types::Address);
    #[doc = "Container type for all return fields from the `getAmountOutMin` function with signature `getAmountOutMin(uint256,address[])` and selector `[72, 51, 68, 41]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct GetAmountOutMinReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `getPrice` function with signature `getPrice()` and selector `[152, 213, 253, 202]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct GetPriceReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `landNft` function with signature `landNft()` and selector `[161, 172, 189, 126]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct LandNftReturn(pub ethers::core::types::Address);
    #[doc = "Container type for all return fields from the `owner` function with signature `owner()` and selector `[141, 165, 203, 91]`"]
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
