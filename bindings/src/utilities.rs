pub use utilities::*;
#[allow(clippy::too_many_arguments, non_camel_case_types)]
pub mod utilities {
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
    #[doc = "Utilities was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static UTILITIES_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            ethers :: core :: utils :: __serde_json :: from_str ("[{\"inputs\":[{\"internalType\":\"string\",\"name\":\"\",\"type\":\"string\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"log\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"log_address\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint256[]\",\"name\":\"val\",\"type\":\"uint256[]\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"log_array\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"int256[]\",\"name\":\"val\",\"type\":\"int256[]\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"log_array\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address[]\",\"name\":\"val\",\"type\":\"address[]\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"log_array\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"bytes\",\"name\":\"\",\"type\":\"bytes\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"log_bytes\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"log_bytes32\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"int256\",\"name\":\"\",\"type\":\"int256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"log_int\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"key\",\"type\":\"string\",\"components\":[],\"indexed\":false},{\"internalType\":\"address\",\"name\":\"val\",\"type\":\"address\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"log_named_address\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"key\",\"type\":\"string\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256[]\",\"name\":\"val\",\"type\":\"uint256[]\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"log_named_array\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"key\",\"type\":\"string\",\"components\":[],\"indexed\":false},{\"internalType\":\"int256[]\",\"name\":\"val\",\"type\":\"int256[]\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"log_named_array\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"key\",\"type\":\"string\",\"components\":[],\"indexed\":false},{\"internalType\":\"address[]\",\"name\":\"val\",\"type\":\"address[]\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"log_named_array\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"key\",\"type\":\"string\",\"components\":[],\"indexed\":false},{\"internalType\":\"bytes\",\"name\":\"val\",\"type\":\"bytes\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"log_named_bytes\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"key\",\"type\":\"string\",\"components\":[],\"indexed\":false},{\"internalType\":\"bytes32\",\"name\":\"val\",\"type\":\"bytes32\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"log_named_bytes32\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"key\",\"type\":\"string\",\"components\":[],\"indexed\":false},{\"internalType\":\"int256\",\"name\":\"val\",\"type\":\"int256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"decimals\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"log_named_decimal_int\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"key\",\"type\":\"string\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"val\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"decimals\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"log_named_decimal_uint\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"key\",\"type\":\"string\",\"components\":[],\"indexed\":false},{\"internalType\":\"int256\",\"name\":\"val\",\"type\":\"int256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"log_named_int\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"key\",\"type\":\"string\",\"components\":[],\"indexed\":false},{\"internalType\":\"string\",\"name\":\"val\",\"type\":\"string\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"log_named_string\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"key\",\"type\":\"string\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"val\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"log_named_uint\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"\",\"type\":\"string\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"log_string\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"log_uint\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"bytes\",\"name\":\"\",\"type\":\"bytes\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"logs\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"IS_SCRIPT\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"IS_TEST\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"userNum\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"createUsers\",\"outputs\":[{\"internalType\":\"address payable[]\",\"name\":\"\",\"type\":\"address[]\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"failed\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"getNextUserAddress\",\"outputs\":[{\"internalType\":\"address payable\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"numBlocks\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"mineBlocks\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"vm\",\"outputs\":[{\"internalType\":\"contract Vm\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]}]") . expect ("invalid abi")
        });
    #[doc = r" Bytecode of the #name contract"]
    pub static UTILITIES_BYTECODE: ethers::contract::Lazy<ethers::core::types::Bytes> =
        ethers::contract::Lazy::new(|| {
            "0x6000805462ff00ff1916620100011790556b75736572206164647265737360a01b60a052600c60805260ac6040527ffadd6953a0436e85528ded789af2e2b7e57c1cd7c68c5c3796d8ea67e0018db760085534801561005d57600080fd5b506106408061006d6000396000f3fe608060405234801561001057600080fd5b506004361061007d5760003560e01c8063ba414fa61161005b578063ba414fa614610106578063f82de7b01461011e578063f8ccbf4714610133578063fa7626d41461014657600080fd5b80633a76846314610082578063792e11f5146100ba578063b90a68fa146100da575b600080fd5b61009d737109709ecfa91a80626ff3989d68f67f5b1dd12d81565b6040516001600160a01b0390911681526020015b60405180910390f35b6100cd6100c8366004610474565b610153565b6040516100b1919061048d565b60088054604080516020808201849052825180830382018152918301909252805191012090915561009d565b61010e6102d1565b60405190151581526020016100b1565b61013161012c366004610474565b6103fc565b005b60005461010e9062010000900460ff1681565b60005461010e9060ff1681565b606060008267ffffffffffffffff811115610170576101706104da565b604051908082528060200260200182016040528015610199578160200160208202803683370190505b50905060005b838110156102ca576000306001600160a01b031663b90a68fa6040518163ffffffff1660e01b81526004016020604051808303816000875af11580156101e9573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019061020d91906104f0565b60405163c88a5e6d60e01b81526001600160a01b038216600482015268056bc75e2d631000006024820152909150737109709ecfa91a80626ff3989d68f67f5b1dd12d9063c88a5e6d90604401600060405180830381600087803b15801561027457600080fd5b505af1158015610288573d6000803e3d6000fd5b505050508083838151811061029f5761029f610520565b6001600160a01b039092166020928302919091019091015250806102c28161054c565b91505061019f565b5092915050565b60008054610100900460ff16156102f15750600054610100900460ff1690565b6000737109709ecfa91a80626ff3989d68f67f5b1dd12d3b156103f75760408051737109709ecfa91a80626ff3989d68f67f5b1dd12d602082018190526519985a5b195960d21b8284015282518083038401815260608301909352600092909161037f917f667f9d70ca411d70ead50d8d5c22070dafc36ad75f3dcf5e7237b22ade9aecc4916080016105a0565b60408051601f1981840301815290829052610399916105c4565b6000604051808303816000865af19150503d80600081146103d6576040519150601f19603f3d011682016040523d82523d6000602084013e6103db565b606091505b50915050808060200190518101906103f391906105d0565b9150505b919050565b600061040882436105f2565b6040516301f7b4f360e41b815260048101829052909150737109709ecfa91a80626ff3989d68f67f5b1dd12d90631f7b4f3090602401600060405180830381600087803b15801561045857600080fd5b505af115801561046c573d6000803e3d6000fd5b505050505050565b60006020828403121561048657600080fd5b5035919050565b6020808252825182820181905260009190848201906040850190845b818110156104ce5783516001600160a01b0316835292840192918401916001016104a9565b50909695505050505050565b634e487b7160e01b600052604160045260246000fd5b60006020828403121561050257600080fd5b81516001600160a01b038116811461051957600080fd5b9392505050565b634e487b7160e01b600052603260045260246000fd5b634e487b7160e01b600052601160045260246000fd5b60006001820161055e5761055e610536565b5060010190565b6000815160005b81811015610586576020818501810151868301520161056c565b81811115610595576000828601525b509290920192915050565b6001600160e01b03198316815260006105bc6004830184610565565b949350505050565b60006105198284610565565b6000602082840312156105e257600080fd5b8151801515811461051957600080fd5b6000821982111561060557610605610536565b50019056fea264697066735822122032e35b9d18f1fd49ce9ed7dd5378b5cec2bca3f7a1c6fff772027eaa2523ca0364736f6c634300080f0033" . parse () . expect ("invalid bytecode")
        });
    pub struct Utilities<M>(ethers::contract::Contract<M>);
    impl<M> Clone for Utilities<M> {
        fn clone(&self) -> Self {
            Utilities(self.0.clone())
        }
    }
    impl<M> std::ops::Deref for Utilities<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers::providers::Middleware> std::fmt::Debug for Utilities<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(Utilities))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ethers::providers::Middleware> Utilities<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(address.into(), UTILITIES_ABI.clone(), client).into()
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
                UTILITIES_ABI.clone(),
                UTILITIES_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        #[doc = "Calls the contract's `IS_SCRIPT` (0xf8ccbf47) function"]
        pub fn is_script(&self) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([248, 204, 191, 71], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `IS_TEST` (0xfa7626d4) function"]
        pub fn is_test(&self) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([250, 118, 38, 212], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `createUsers` (0x792e11f5) function"]
        pub fn create_users(
            &self,
            user_num: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<
            M,
            ::std::vec::Vec<ethers::core::types::Address>,
        > {
            self.0
                .method_hash([121, 46, 17, 245], user_num)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `failed` (0xba414fa6) function"]
        pub fn failed(&self) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([186, 65, 79, 166], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getNextUserAddress` (0xb90a68fa) function"]
        pub fn get_next_user_address(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([185, 10, 104, 250], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `mineBlocks` (0xf82de7b0) function"]
        pub fn mine_blocks(
            &self,
            num_blocks: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([248, 45, 231, 176], num_blocks)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `vm` (0x3a768463) function"]
        pub fn vm(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([58, 118, 132, 99], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Gets the contract's `log` event"]
        pub fn log_filter(&self) -> ethers::contract::builders::Event<M, LogFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `log_address` event"]
        pub fn log_address_filter(&self) -> ethers::contract::builders::Event<M, LogAddressFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `log_array` event"]
        pub fn log_array_1_filter(&self) -> ethers::contract::builders::Event<M, LogArray1Filter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `log_array` event"]
        pub fn log_array_2_filter(&self) -> ethers::contract::builders::Event<M, LogArray2Filter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `log_array` event"]
        pub fn log_array_3_filter(&self) -> ethers::contract::builders::Event<M, LogArray3Filter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `log_bytes` event"]
        pub fn log_bytes_filter(&self) -> ethers::contract::builders::Event<M, LogBytesFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `log_bytes32` event"]
        pub fn log_bytes_32_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, LogBytes32Filter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `log_int` event"]
        pub fn log_int_filter(&self) -> ethers::contract::builders::Event<M, LogIntFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `log_named_address` event"]
        pub fn log_named_address_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, LogNamedAddressFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `log_named_array` event"]
        pub fn log_named_array_1_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, LogNamedArray1Filter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `log_named_array` event"]
        pub fn log_named_array_2_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, LogNamedArray2Filter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `log_named_array` event"]
        pub fn log_named_array_3_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, LogNamedArray3Filter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `log_named_bytes` event"]
        pub fn log_named_bytes_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, LogNamedBytesFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `log_named_bytes32` event"]
        pub fn log_named_bytes_32_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, LogNamedBytes32Filter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `log_named_decimal_int` event"]
        pub fn log_named_decimal_int_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, LogNamedDecimalIntFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `log_named_decimal_uint` event"]
        pub fn log_named_decimal_uint_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, LogNamedDecimalUintFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `log_named_int` event"]
        pub fn log_named_int_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, LogNamedIntFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `log_named_string` event"]
        pub fn log_named_string_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, LogNamedStringFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `log_named_uint` event"]
        pub fn log_named_uint_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, LogNamedUintFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `log_string` event"]
        pub fn log_string_filter(&self) -> ethers::contract::builders::Event<M, LogStringFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `log_uint` event"]
        pub fn log_uint_filter(&self) -> ethers::contract::builders::Event<M, LogUintFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `logs` event"]
        pub fn logs_filter(&self) -> ethers::contract::builders::Event<M, LogsFilter> {
            self.0.event()
        }
        #[doc = r" Returns an [`Event`](#ethers_contract::builders::Event) builder for all events of this contract"]
        pub fn events(&self) -> ethers::contract::builders::Event<M, UtilitiesEvents> {
            self.0.event_with_filter(Default::default())
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>> for Utilities<M> {
        fn from(contract: ethers::contract::Contract<M>) -> Self {
            Self(contract)
        }
    }
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
    )]
    #[ethevent(name = "log", abi = "log(string)")]
    pub struct LogFilter(pub String);
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
    )]
    #[ethevent(name = "log_address", abi = "log_address(address)")]
    pub struct LogAddressFilter(pub ethers::core::types::Address);
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
    )]
    #[ethevent(name = "log_array", abi = "log_array(uint256[])")]
    pub struct LogArray1Filter {
        pub val: Vec<ethers::core::types::U256>,
    }
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
    )]
    #[ethevent(name = "log_array", abi = "log_array(int256[])")]
    pub struct LogArray2Filter {
        pub val: Vec<I256>,
    }
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
    )]
    #[ethevent(name = "log_array", abi = "log_array(address[])")]
    pub struct LogArray3Filter {
        pub val: Vec<ethers::core::types::Address>,
    }
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
    )]
    #[ethevent(name = "log_bytes", abi = "log_bytes(bytes)")]
    pub struct LogBytesFilter(pub ethers::core::types::Bytes);
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
    )]
    #[ethevent(name = "log_bytes32", abi = "log_bytes32(bytes32)")]
    pub struct LogBytes32Filter(pub [u8; 32]);
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
    )]
    #[ethevent(name = "log_int", abi = "log_int(int256)")]
    pub struct LogIntFilter(pub I256);
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
    )]
    #[ethevent(name = "log_named_address", abi = "log_named_address(string,address)")]
    pub struct LogNamedAddressFilter {
        pub key: String,
        pub val: ethers::core::types::Address,
    }
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
    )]
    #[ethevent(name = "log_named_array", abi = "log_named_array(string,uint256[])")]
    pub struct LogNamedArray1Filter {
        pub key: String,
        pub val: Vec<ethers::core::types::U256>,
    }
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
    )]
    #[ethevent(name = "log_named_array", abi = "log_named_array(string,int256[])")]
    pub struct LogNamedArray2Filter {
        pub key: String,
        pub val: Vec<I256>,
    }
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
    )]
    #[ethevent(name = "log_named_array", abi = "log_named_array(string,address[])")]
    pub struct LogNamedArray3Filter {
        pub key: String,
        pub val: Vec<ethers::core::types::Address>,
    }
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
    )]
    #[ethevent(name = "log_named_bytes", abi = "log_named_bytes(string,bytes)")]
    pub struct LogNamedBytesFilter {
        pub key: String,
        pub val: ethers::core::types::Bytes,
    }
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
    )]
    #[ethevent(name = "log_named_bytes32", abi = "log_named_bytes32(string,bytes32)")]
    pub struct LogNamedBytes32Filter {
        pub key: String,
        pub val: [u8; 32],
    }
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
    )]
    #[ethevent(
        name = "log_named_decimal_int",
        abi = "log_named_decimal_int(string,int256,uint256)"
    )]
    pub struct LogNamedDecimalIntFilter {
        pub key: String,
        pub val: I256,
        pub decimals: ethers::core::types::U256,
    }
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
    )]
    #[ethevent(
        name = "log_named_decimal_uint",
        abi = "log_named_decimal_uint(string,uint256,uint256)"
    )]
    pub struct LogNamedDecimalUintFilter {
        pub key: String,
        pub val: ethers::core::types::U256,
        pub decimals: ethers::core::types::U256,
    }
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
    )]
    #[ethevent(name = "log_named_int", abi = "log_named_int(string,int256)")]
    pub struct LogNamedIntFilter {
        pub key: String,
        pub val: I256,
    }
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
    )]
    #[ethevent(name = "log_named_string", abi = "log_named_string(string,string)")]
    pub struct LogNamedStringFilter {
        pub key: String,
        pub val: String,
    }
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
    )]
    #[ethevent(name = "log_named_uint", abi = "log_named_uint(string,uint256)")]
    pub struct LogNamedUintFilter {
        pub key: String,
        pub val: ethers::core::types::U256,
    }
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
    )]
    #[ethevent(name = "log_string", abi = "log_string(string)")]
    pub struct LogStringFilter(pub String);
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
    )]
    #[ethevent(name = "log_uint", abi = "log_uint(uint256)")]
    pub struct LogUintFilter(pub ethers::core::types::U256);
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
    )]
    #[ethevent(name = "logs", abi = "logs(bytes)")]
    pub struct LogsFilter(pub ethers::core::types::Bytes);
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum UtilitiesEvents {
        LogFilter(LogFilter),
        LogAddressFilter(LogAddressFilter),
        LogArray1Filter(LogArray1Filter),
        LogArray2Filter(LogArray2Filter),
        LogArray3Filter(LogArray3Filter),
        LogBytesFilter(LogBytesFilter),
        LogBytes32Filter(LogBytes32Filter),
        LogIntFilter(LogIntFilter),
        LogNamedAddressFilter(LogNamedAddressFilter),
        LogNamedArray1Filter(LogNamedArray1Filter),
        LogNamedArray2Filter(LogNamedArray2Filter),
        LogNamedArray3Filter(LogNamedArray3Filter),
        LogNamedBytesFilter(LogNamedBytesFilter),
        LogNamedBytes32Filter(LogNamedBytes32Filter),
        LogNamedDecimalIntFilter(LogNamedDecimalIntFilter),
        LogNamedDecimalUintFilter(LogNamedDecimalUintFilter),
        LogNamedIntFilter(LogNamedIntFilter),
        LogNamedStringFilter(LogNamedStringFilter),
        LogNamedUintFilter(LogNamedUintFilter),
        LogStringFilter(LogStringFilter),
        LogUintFilter(LogUintFilter),
        LogsFilter(LogsFilter),
    }
    impl ethers::contract::EthLogDecode for UtilitiesEvents {
        fn decode_log(
            log: &ethers::core::abi::RawLog,
        ) -> ::std::result::Result<Self, ethers::core::abi::Error>
        where
            Self: Sized,
        {
            if let Ok(decoded) = LogFilter::decode_log(log) {
                return Ok(UtilitiesEvents::LogFilter(decoded));
            }
            if let Ok(decoded) = LogAddressFilter::decode_log(log) {
                return Ok(UtilitiesEvents::LogAddressFilter(decoded));
            }
            if let Ok(decoded) = LogArray1Filter::decode_log(log) {
                return Ok(UtilitiesEvents::LogArray1Filter(decoded));
            }
            if let Ok(decoded) = LogArray2Filter::decode_log(log) {
                return Ok(UtilitiesEvents::LogArray2Filter(decoded));
            }
            if let Ok(decoded) = LogArray3Filter::decode_log(log) {
                return Ok(UtilitiesEvents::LogArray3Filter(decoded));
            }
            if let Ok(decoded) = LogBytesFilter::decode_log(log) {
                return Ok(UtilitiesEvents::LogBytesFilter(decoded));
            }
            if let Ok(decoded) = LogBytes32Filter::decode_log(log) {
                return Ok(UtilitiesEvents::LogBytes32Filter(decoded));
            }
            if let Ok(decoded) = LogIntFilter::decode_log(log) {
                return Ok(UtilitiesEvents::LogIntFilter(decoded));
            }
            if let Ok(decoded) = LogNamedAddressFilter::decode_log(log) {
                return Ok(UtilitiesEvents::LogNamedAddressFilter(decoded));
            }
            if let Ok(decoded) = LogNamedArray1Filter::decode_log(log) {
                return Ok(UtilitiesEvents::LogNamedArray1Filter(decoded));
            }
            if let Ok(decoded) = LogNamedArray2Filter::decode_log(log) {
                return Ok(UtilitiesEvents::LogNamedArray2Filter(decoded));
            }
            if let Ok(decoded) = LogNamedArray3Filter::decode_log(log) {
                return Ok(UtilitiesEvents::LogNamedArray3Filter(decoded));
            }
            if let Ok(decoded) = LogNamedBytesFilter::decode_log(log) {
                return Ok(UtilitiesEvents::LogNamedBytesFilter(decoded));
            }
            if let Ok(decoded) = LogNamedBytes32Filter::decode_log(log) {
                return Ok(UtilitiesEvents::LogNamedBytes32Filter(decoded));
            }
            if let Ok(decoded) = LogNamedDecimalIntFilter::decode_log(log) {
                return Ok(UtilitiesEvents::LogNamedDecimalIntFilter(decoded));
            }
            if let Ok(decoded) = LogNamedDecimalUintFilter::decode_log(log) {
                return Ok(UtilitiesEvents::LogNamedDecimalUintFilter(decoded));
            }
            if let Ok(decoded) = LogNamedIntFilter::decode_log(log) {
                return Ok(UtilitiesEvents::LogNamedIntFilter(decoded));
            }
            if let Ok(decoded) = LogNamedStringFilter::decode_log(log) {
                return Ok(UtilitiesEvents::LogNamedStringFilter(decoded));
            }
            if let Ok(decoded) = LogNamedUintFilter::decode_log(log) {
                return Ok(UtilitiesEvents::LogNamedUintFilter(decoded));
            }
            if let Ok(decoded) = LogStringFilter::decode_log(log) {
                return Ok(UtilitiesEvents::LogStringFilter(decoded));
            }
            if let Ok(decoded) = LogUintFilter::decode_log(log) {
                return Ok(UtilitiesEvents::LogUintFilter(decoded));
            }
            if let Ok(decoded) = LogsFilter::decode_log(log) {
                return Ok(UtilitiesEvents::LogsFilter(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::std::fmt::Display for UtilitiesEvents {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                UtilitiesEvents::LogFilter(element) => element.fmt(f),
                UtilitiesEvents::LogAddressFilter(element) => element.fmt(f),
                UtilitiesEvents::LogArray1Filter(element) => element.fmt(f),
                UtilitiesEvents::LogArray2Filter(element) => element.fmt(f),
                UtilitiesEvents::LogArray3Filter(element) => element.fmt(f),
                UtilitiesEvents::LogBytesFilter(element) => element.fmt(f),
                UtilitiesEvents::LogBytes32Filter(element) => element.fmt(f),
                UtilitiesEvents::LogIntFilter(element) => element.fmt(f),
                UtilitiesEvents::LogNamedAddressFilter(element) => element.fmt(f),
                UtilitiesEvents::LogNamedArray1Filter(element) => element.fmt(f),
                UtilitiesEvents::LogNamedArray2Filter(element) => element.fmt(f),
                UtilitiesEvents::LogNamedArray3Filter(element) => element.fmt(f),
                UtilitiesEvents::LogNamedBytesFilter(element) => element.fmt(f),
                UtilitiesEvents::LogNamedBytes32Filter(element) => element.fmt(f),
                UtilitiesEvents::LogNamedDecimalIntFilter(element) => element.fmt(f),
                UtilitiesEvents::LogNamedDecimalUintFilter(element) => element.fmt(f),
                UtilitiesEvents::LogNamedIntFilter(element) => element.fmt(f),
                UtilitiesEvents::LogNamedStringFilter(element) => element.fmt(f),
                UtilitiesEvents::LogNamedUintFilter(element) => element.fmt(f),
                UtilitiesEvents::LogStringFilter(element) => element.fmt(f),
                UtilitiesEvents::LogUintFilter(element) => element.fmt(f),
                UtilitiesEvents::LogsFilter(element) => element.fmt(f),
            }
        }
    }
    #[doc = "Container type for all input parameters for the `IS_SCRIPT` function with signature `IS_SCRIPT()` and selector `[248, 204, 191, 71]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "IS_SCRIPT", abi = "IS_SCRIPT()")]
    pub struct IsScriptCall;
    #[doc = "Container type for all input parameters for the `IS_TEST` function with signature `IS_TEST()` and selector `[250, 118, 38, 212]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "IS_TEST", abi = "IS_TEST()")]
    pub struct IsTestCall;
    #[doc = "Container type for all input parameters for the `createUsers` function with signature `createUsers(uint256)` and selector `[121, 46, 17, 245]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "createUsers", abi = "createUsers(uint256)")]
    pub struct CreateUsersCall {
        pub user_num: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `failed` function with signature `failed()` and selector `[186, 65, 79, 166]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "failed", abi = "failed()")]
    pub struct FailedCall;
    #[doc = "Container type for all input parameters for the `getNextUserAddress` function with signature `getNextUserAddress()` and selector `[185, 10, 104, 250]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "getNextUserAddress", abi = "getNextUserAddress()")]
    pub struct GetNextUserAddressCall;
    #[doc = "Container type for all input parameters for the `mineBlocks` function with signature `mineBlocks(uint256)` and selector `[248, 45, 231, 176]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "mineBlocks", abi = "mineBlocks(uint256)")]
    pub struct MineBlocksCall {
        pub num_blocks: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `vm` function with signature `vm()` and selector `[58, 118, 132, 99]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "vm", abi = "vm()")]
    pub struct VmCall;
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum UtilitiesCalls {
        IsScript(IsScriptCall),
        IsTest(IsTestCall),
        CreateUsers(CreateUsersCall),
        Failed(FailedCall),
        GetNextUserAddress(GetNextUserAddressCall),
        MineBlocks(MineBlocksCall),
        Vm(VmCall),
    }
    impl ethers::core::abi::AbiDecode for UtilitiesCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::std::result::Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <IsScriptCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(UtilitiesCalls::IsScript(decoded));
            }
            if let Ok(decoded) = <IsTestCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(UtilitiesCalls::IsTest(decoded));
            }
            if let Ok(decoded) =
                <CreateUsersCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(UtilitiesCalls::CreateUsers(decoded));
            }
            if let Ok(decoded) = <FailedCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(UtilitiesCalls::Failed(decoded));
            }
            if let Ok(decoded) =
                <GetNextUserAddressCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(UtilitiesCalls::GetNextUserAddress(decoded));
            }
            if let Ok(decoded) =
                <MineBlocksCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(UtilitiesCalls::MineBlocks(decoded));
            }
            if let Ok(decoded) = <VmCall as ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(UtilitiesCalls::Vm(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for UtilitiesCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                UtilitiesCalls::IsScript(element) => element.encode(),
                UtilitiesCalls::IsTest(element) => element.encode(),
                UtilitiesCalls::CreateUsers(element) => element.encode(),
                UtilitiesCalls::Failed(element) => element.encode(),
                UtilitiesCalls::GetNextUserAddress(element) => element.encode(),
                UtilitiesCalls::MineBlocks(element) => element.encode(),
                UtilitiesCalls::Vm(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for UtilitiesCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                UtilitiesCalls::IsScript(element) => element.fmt(f),
                UtilitiesCalls::IsTest(element) => element.fmt(f),
                UtilitiesCalls::CreateUsers(element) => element.fmt(f),
                UtilitiesCalls::Failed(element) => element.fmt(f),
                UtilitiesCalls::GetNextUserAddress(element) => element.fmt(f),
                UtilitiesCalls::MineBlocks(element) => element.fmt(f),
                UtilitiesCalls::Vm(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<IsScriptCall> for UtilitiesCalls {
        fn from(var: IsScriptCall) -> Self {
            UtilitiesCalls::IsScript(var)
        }
    }
    impl ::std::convert::From<IsTestCall> for UtilitiesCalls {
        fn from(var: IsTestCall) -> Self {
            UtilitiesCalls::IsTest(var)
        }
    }
    impl ::std::convert::From<CreateUsersCall> for UtilitiesCalls {
        fn from(var: CreateUsersCall) -> Self {
            UtilitiesCalls::CreateUsers(var)
        }
    }
    impl ::std::convert::From<FailedCall> for UtilitiesCalls {
        fn from(var: FailedCall) -> Self {
            UtilitiesCalls::Failed(var)
        }
    }
    impl ::std::convert::From<GetNextUserAddressCall> for UtilitiesCalls {
        fn from(var: GetNextUserAddressCall) -> Self {
            UtilitiesCalls::GetNextUserAddress(var)
        }
    }
    impl ::std::convert::From<MineBlocksCall> for UtilitiesCalls {
        fn from(var: MineBlocksCall) -> Self {
            UtilitiesCalls::MineBlocks(var)
        }
    }
    impl ::std::convert::From<VmCall> for UtilitiesCalls {
        fn from(var: VmCall) -> Self {
            UtilitiesCalls::Vm(var)
        }
    }
    #[doc = "Container type for all return fields from the `IS_SCRIPT` function with signature `IS_SCRIPT()` and selector `[248, 204, 191, 71]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct IsScriptReturn(pub bool);
    #[doc = "Container type for all return fields from the `IS_TEST` function with signature `IS_TEST()` and selector `[250, 118, 38, 212]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct IsTestReturn(pub bool);
    #[doc = "Container type for all return fields from the `createUsers` function with signature `createUsers(uint256)` and selector `[121, 46, 17, 245]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct CreateUsersReturn(pub ::std::vec::Vec<ethers::core::types::Address>);
    #[doc = "Container type for all return fields from the `failed` function with signature `failed()` and selector `[186, 65, 79, 166]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct FailedReturn(pub bool);
    #[doc = "Container type for all return fields from the `getNextUserAddress` function with signature `getNextUserAddress()` and selector `[185, 10, 104, 250]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct GetNextUserAddressReturn(pub ethers::core::types::Address);
    #[doc = "Container type for all return fields from the `vm` function with signature `vm()` and selector `[58, 118, 132, 99]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct VmReturn(pub ethers::core::types::Address);
}
