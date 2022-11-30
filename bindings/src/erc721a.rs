pub use erc721a::*;
#[allow(clippy::too_many_arguments, non_camel_case_types)]
pub mod erc721a {
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
    #[doc = "ERC721A was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    # [rustfmt :: skip] const __ABI : & str = "[{\"inputs\":[{\"internalType\":\"string\",\"name\":\"name_\",\"type\":\"string\",\"components\":[]},{\"internalType\":\"string\",\"name\":\"symbol_\",\"type\":\"string\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"constructor\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"ApprovalCallerNotOwnerNorApproved\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"ApprovalQueryForNonexistentToken\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"ApproveToCaller\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"BalanceQueryForZeroAddress\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"MintERC2309QuantityExceedsLimit\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"MintToZeroAddress\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"MintZeroQuantity\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"OwnerQueryForNonexistentToken\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"OwnershipNotInitializedForExtraData\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"TransferCallerNotOwnerNorApproved\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"TransferFromIncorrectOwner\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"TransferToNonERC721ReceiverImplementer\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"TransferToZeroAddress\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"URIQueryForNonexistentToken\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"owner\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"approved\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"tokenId\",\"type\":\"uint256\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"Approval\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"owner\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"operator\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"bool\",\"name\":\"approved\",\"type\":\"bool\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"ApprovalForAll\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"fromTokenId\",\"type\":\"uint256\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"toTokenId\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"address\",\"name\":\"from\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"to\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"ConsecutiveTransfer\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"from\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"to\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"tokenId\",\"type\":\"uint256\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"Transfer\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"to\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"tokenId\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"approve\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"owner\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"balanceOf\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"tokenId\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getApproved\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"owner\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"operator\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"isApprovedForAll\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"name\",\"outputs\":[{\"internalType\":\"string\",\"name\":\"\",\"type\":\"string\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"tokenId\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"ownerOf\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"from\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"to\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"tokenId\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"safeTransferFrom\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"from\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"to\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"tokenId\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"_data\",\"type\":\"bytes\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"safeTransferFrom\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"operator\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bool\",\"name\":\"approved\",\"type\":\"bool\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setApprovalForAll\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"bytes4\",\"name\":\"interfaceId\",\"type\":\"bytes4\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"supportsInterface\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"symbol\",\"outputs\":[{\"internalType\":\"string\",\"name\":\"\",\"type\":\"string\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"tokenId\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"tokenURI\",\"outputs\":[{\"internalType\":\"string\",\"name\":\"\",\"type\":\"string\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"totalSupply\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"from\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"to\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"tokenId\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"transferFrom\",\"outputs\":[]}]" ;
    #[doc = r" The parsed JSON-ABI of the contract."]
    pub static ERC721A_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            ethers::core::utils::__serde_json::from_str(__ABI).expect("invalid abi")
        });
    #[doc = r" Bytecode of the #name contract"]
    pub static ERC721A_BYTECODE: ethers::contract::Lazy<ethers::core::types::Bytes> =
        ethers::contract::Lazy::new(|| {
            "0x60806040523480156200001157600080fd5b506040516200101d3803806200101d83398101604081905262000034916200012b565b600262000042838262000224565b50600362000051828262000224565b50506000805550620002f0565b634e487b7160e01b600052604160045260246000fd5b600082601f8301126200008657600080fd5b81516001600160401b0380821115620000a357620000a36200005e565b604051601f8301601f19908116603f01168101908282118183101715620000ce57620000ce6200005e565b81604052838152602092508683858801011115620000eb57600080fd5b600091505b838210156200010f5785820183015181830184015290820190620000f0565b83821115620001215760008385830101525b9695505050505050565b600080604083850312156200013f57600080fd5b82516001600160401b03808211156200015757600080fd5b620001658683870162000074565b935060208501519150808211156200017c57600080fd5b506200018b8582860162000074565b9150509250929050565b600181811c90821680620001aa57607f821691505b602082108103620001cb57634e487b7160e01b600052602260045260246000fd5b50919050565b601f8211156200021f57600081815260208120601f850160051c81016020861015620001fa5750805b601f850160051c820191505b818110156200021b5782815560010162000206565b5050505b505050565b81516001600160401b038111156200024057620002406200005e565b620002588162000251845462000195565b84620001d1565b602080601f831160018114620002905760008415620002775750858301515b600019600386901b1c1916600185901b1785556200021b565b600085815260208120601f198616915b82811015620002c157888601518255948401946001909101908401620002a0565b5085821015620002e05787850151600019600388901b60f8161c191681555b5050505050600190811b01905550565b610d1d80620003006000396000f3fe608060405234801561001057600080fd5b50600436106100ea5760003560e01c80636352211e1161008c578063a22cb46511610066578063a22cb465146101d6578063b88d4fde146101e9578063c87b56dd146101fc578063e985e9c51461020f57600080fd5b80636352211e146101a857806370a08231146101bb57806395d89b41146101ce57600080fd5b8063095ea7b3116100c8578063095ea7b31461015757806318160ddd1461016c57806323b872dd1461018257806342842e0e1461019557600080fd5b806301ffc9a7146100ef57806306fdde0314610117578063081812fc1461012c575b600080fd5b6101026100fd366004610985565b61024b565b60405190151581526020015b60405180910390f35b61011f61029d565b60405161010e91906109fa565b61013f61013a366004610a0d565b61032f565b6040516001600160a01b03909116815260200161010e565b61016a610165366004610a42565b610373565b005b600154600054035b60405190815260200161010e565b61016a610190366004610a6c565b610413565b61016a6101a3366004610a6c565b6105ab565b61013f6101b6366004610a0d565b6105cb565b6101746101c9366004610aa8565b6105d6565b61011f610625565b61016a6101e4366004610ac3565b610634565b61016a6101f7366004610b15565b6106c9565b61011f61020a366004610a0d565b610713565b61010261021d366004610bf1565b6001600160a01b03918216600090815260076020908152604080832093909416825291909152205460ff1690565b60006301ffc9a760e01b6001600160e01b03198316148061027c57506380ac58cd60e01b6001600160e01b03198316145b806102975750635b5e139f60e01b6001600160e01b03198316145b92915050565b6060600280546102ac90610c24565b80601f01602080910402602001604051908101604052809291908181526020018280546102d890610c24565b80156103255780601f106102fa57610100808354040283529160200191610325565b820191906000526020600020905b81548152906001019060200180831161030857829003601f168201915b5050505050905090565b600061033a826107a4565b610357576040516333d1c03960e21b815260040160405180910390fd5b506000908152600660205260409020546001600160a01b031690565b600061037e826105cb565b9050336001600160a01b038216146103b75761039a813361021d565b6103b7576040516367d9dca160e11b815260040160405180910390fd5b60008281526006602052604080822080546001600160a01b0319166001600160a01b0387811691821790925591518593918516917f8c5be1e5ebec7d5bd14f71427d1e84f3dd0314c0f7b2291e5b200ac8c7c3b92591a4505050565b600061041e826107cb565b9050836001600160a01b0316816001600160a01b0316146104515760405162a1148160e81b815260040160405180910390fd5b60008281526006602052604090208054338082146001600160a01b0388169091141761049e57610481863361021d565b61049e57604051632ce44b5f60e11b815260040160405180910390fd5b6001600160a01b0385166104c557604051633a954ecd60e21b815260040160405180910390fd5b80156104d057600082555b6001600160a01b038681166000908152600560205260408082208054600019019055918716808252919020805460010190554260a01b17600160e11b17600085815260046020526040812091909155600160e11b84169003610562576001840160008181526004602052604081205490036105605760005481146105605760008181526004602052604090208490555b505b83856001600160a01b0316876001600160a01b03167fddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef60405160405180910390a4505050505050565b6105c6838383604051806020016040528060008152506106c9565b505050565b6000610297826107cb565b60006001600160a01b0382166105ff576040516323d3ad8160e21b815260040160405180910390fd5b506001600160a01b031660009081526005602052604090205467ffffffffffffffff1690565b6060600380546102ac90610c24565b336001600160a01b0383160361065d5760405163b06307db60e01b815260040160405180910390fd5b3360008181526007602090815260408083206001600160a01b03871680855290835292819020805460ff191686151590811790915590519081529192917f17307eab39ab6107e8899845ad3d59bd9653f200f220920489ca2b5937696c31910160405180910390a35050565b6106d4848484610413565b6001600160a01b0383163b1561070d576106f084848484610832565b61070d576040516368d2bf6b60e11b815260040160405180910390fd5b50505050565b606061071e826107a4565b61073b57604051630a14c4b560e41b815260040160405180910390fd5b600061075260408051602081019091526000815290565b90508051600003610772576040518060200160405280600081525061079d565b8061077c8461091d565b60405160200161078d929190610c5e565b6040516020818303038152906040525b9392505050565b6000805482108015610297575050600090815260046020526040902054600160e01b161590565b6000816000548110156108195760008181526004602052604081205490600160e01b82169003610817575b8060000361079d5750600019016000818152600460205260409020546107f6565b505b604051636f96cda160e11b815260040160405180910390fd5b604051630a85bd0160e11b81526000906001600160a01b0385169063150b7a0290610867903390899088908890600401610c8d565b6020604051808303816000875af19250505080156108a2575060408051601f3d908101601f1916820190925261089f91810190610cca565b60015b610900573d8080156108d0576040519150601f19603f3d011682016040523d82523d6000602084013e6108d5565b606091505b5080516000036108f8576040516368d2bf6b60e11b815260040160405180910390fd5b805181602001fd5b6001600160e01b031916630a85bd0160e11b149050949350505050565b604080516080810191829052607f0190826030600a8206018353600a90045b801561095a57600183039250600a81066030018353600a900461093c565b50819003601f19909101908152919050565b6001600160e01b03198116811461098257600080fd5b50565b60006020828403121561099757600080fd5b813561079d8161096c565b60005b838110156109bd5781810151838201526020016109a5565b8381111561070d5750506000910152565b600081518084526109e68160208601602086016109a2565b601f01601f19169290920160200192915050565b60208152600061079d60208301846109ce565b600060208284031215610a1f57600080fd5b5035919050565b80356001600160a01b0381168114610a3d57600080fd5b919050565b60008060408385031215610a5557600080fd5b610a5e83610a26565b946020939093013593505050565b600080600060608486031215610a8157600080fd5b610a8a84610a26565b9250610a9860208501610a26565b9150604084013590509250925092565b600060208284031215610aba57600080fd5b61079d82610a26565b60008060408385031215610ad657600080fd5b610adf83610a26565b915060208301358015158114610af457600080fd5b809150509250929050565b634e487b7160e01b600052604160045260246000fd5b60008060008060808587031215610b2b57600080fd5b610b3485610a26565b9350610b4260208601610a26565b925060408501359150606085013567ffffffffffffffff80821115610b6657600080fd5b818701915087601f830112610b7a57600080fd5b813581811115610b8c57610b8c610aff565b604051601f8201601f19908116603f01168101908382118183101715610bb457610bb4610aff565b816040528281528a6020848701011115610bcd57600080fd5b82602086016020830137600060208483010152809550505050505092959194509250565b60008060408385031215610c0457600080fd5b610c0d83610a26565b9150610c1b60208401610a26565b90509250929050565b600181811c90821680610c3857607f821691505b602082108103610c5857634e487b7160e01b600052602260045260246000fd5b50919050565b60008351610c708184602088016109a2565b835190830190610c848183602088016109a2565b01949350505050565b6001600160a01b0385811682528416602082015260408101839052608060608201819052600090610cc0908301846109ce565b9695505050505050565b600060208284031215610cdc57600080fd5b815161079d8161096c56fea26469706673582212202cca999da58869bf687d1321a141bf9926f0d6b97c4bb0ae8a1386ee99461c7064736f6c634300080f0033" . parse () . expect ("invalid bytecode")
        });
    pub struct ERC721A<M>(ethers::contract::Contract<M>);
    impl<M> Clone for ERC721A<M> {
        fn clone(&self) -> Self {
            ERC721A(self.0.clone())
        }
    }
    impl<M> std::ops::Deref for ERC721A<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> std::fmt::Debug for ERC721A<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(ERC721A))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ethers::providers::Middleware> ERC721A<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(address.into(), ERC721A_ABI.clone(), client).into()
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
                ERC721A_ABI.clone(),
                ERC721A_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        #[doc = "Calls the contract's `approve` (0x095ea7b3) function"]
        pub fn approve(
            &self,
            to: ethers::core::types::Address,
            token_id: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([9, 94, 167, 179], (to, token_id))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `balanceOf` (0x70a08231) function"]
        pub fn balance_of(
            &self,
            owner: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([112, 160, 130, 49], owner)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getApproved` (0x081812fc) function"]
        pub fn get_approved(
            &self,
            token_id: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([8, 24, 18, 252], token_id)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `isApprovedForAll` (0xe985e9c5) function"]
        pub fn is_approved_for_all(
            &self,
            owner: ethers::core::types::Address,
            operator: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([233, 133, 233, 197], (owner, operator))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `name` (0x06fdde03) function"]
        pub fn name(&self) -> ethers::contract::builders::ContractCall<M, String> {
            self.0
                .method_hash([6, 253, 222, 3], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `ownerOf` (0x6352211e) function"]
        pub fn owner_of(
            &self,
            token_id: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([99, 82, 33, 30], token_id)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `safeTransferFrom` (0x42842e0e) function"]
        pub fn safe_transfer_from(
            &self,
            from: ethers::core::types::Address,
            to: ethers::core::types::Address,
            token_id: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([66, 132, 46, 14], (from, to, token_id))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `safeTransferFrom` (0xb88d4fde) function"]
        pub fn safe_transfer_from_with_from_and_to_and_data(
            &self,
            from: ethers::core::types::Address,
            to: ethers::core::types::Address,
            token_id: ethers::core::types::U256,
            data: ethers::core::types::Bytes,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([184, 141, 79, 222], (from, to, token_id, data))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setApprovalForAll` (0xa22cb465) function"]
        pub fn set_approval_for_all(
            &self,
            operator: ethers::core::types::Address,
            approved: bool,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([162, 44, 180, 101], (operator, approved))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `supportsInterface` (0x01ffc9a7) function"]
        pub fn supports_interface(
            &self,
            interface_id: [u8; 4],
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([1, 255, 201, 167], interface_id)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `symbol` (0x95d89b41) function"]
        pub fn symbol(&self) -> ethers::contract::builders::ContractCall<M, String> {
            self.0
                .method_hash([149, 216, 155, 65], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `tokenURI` (0xc87b56dd) function"]
        pub fn token_uri(
            &self,
            token_id: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, String> {
            self.0
                .method_hash([200, 123, 86, 221], token_id)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `totalSupply` (0x18160ddd) function"]
        pub fn total_supply(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([24, 22, 13, 221], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `transferFrom` (0x23b872dd) function"]
        pub fn transfer_from(
            &self,
            from: ethers::core::types::Address,
            to: ethers::core::types::Address,
            token_id: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([35, 184, 114, 221], (from, to, token_id))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Gets the contract's `Approval` event"]
        pub fn approval_filter(&self) -> ethers::contract::builders::Event<M, ApprovalFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `ApprovalForAll` event"]
        pub fn approval_for_all_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, ApprovalForAllFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `ConsecutiveTransfer` event"]
        pub fn consecutive_transfer_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, ConsecutiveTransferFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `Transfer` event"]
        pub fn transfer_filter(&self) -> ethers::contract::builders::Event<M, TransferFilter> {
            self.0.event()
        }
        #[doc = r" Returns an [`Event`](#ethers_contract::builders::Event) builder for all events of this contract"]
        pub fn events(&self) -> ethers::contract::builders::Event<M, ERC721AEvents> {
            self.0.event_with_filter(Default::default())
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>> for ERC721A<M> {
        fn from(contract: ethers::contract::Contract<M>) -> Self {
            Self(contract)
        }
    }
    #[doc = "Custom Error type `ApprovalCallerNotOwnerNorApproved` with signature `ApprovalCallerNotOwnerNorApproved()` and selector `[207, 179, 185, 66]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthError,
        ethers :: contract :: EthDisplay,
    )]
    #[etherror(
        name = "ApprovalCallerNotOwnerNorApproved",
        abi = "ApprovalCallerNotOwnerNorApproved()"
    )]
    pub struct ApprovalCallerNotOwnerNorApproved;
    #[doc = "Custom Error type `ApprovalQueryForNonexistentToken` with signature `ApprovalQueryForNonexistentToken()` and selector `[207, 71, 0, 228]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthError,
        ethers :: contract :: EthDisplay,
    )]
    #[etherror(
        name = "ApprovalQueryForNonexistentToken",
        abi = "ApprovalQueryForNonexistentToken()"
    )]
    pub struct ApprovalQueryForNonexistentToken;
    #[doc = "Custom Error type `ApproveToCaller` with signature `ApproveToCaller()` and selector `[176, 99, 7, 219]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthError,
        ethers :: contract :: EthDisplay,
    )]
    #[etherror(name = "ApproveToCaller", abi = "ApproveToCaller()")]
    pub struct ApproveToCaller;
    #[doc = "Custom Error type `BalanceQueryForZeroAddress` with signature `BalanceQueryForZeroAddress()` and selector `[143, 78, 182, 4]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthError,
        ethers :: contract :: EthDisplay,
    )]
    #[etherror(
        name = "BalanceQueryForZeroAddress",
        abi = "BalanceQueryForZeroAddress()"
    )]
    pub struct BalanceQueryForZeroAddress;
    #[doc = "Custom Error type `MintERC2309QuantityExceedsLimit` with signature `MintERC2309QuantityExceedsLimit()` and selector `[61, 177, 249, 175]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthError,
        ethers :: contract :: EthDisplay,
    )]
    #[etherror(
        name = "MintERC2309QuantityExceedsLimit",
        abi = "MintERC2309QuantityExceedsLimit()"
    )]
    pub struct MintERC2309QuantityExceedsLimit;
    #[doc = "Custom Error type `MintToZeroAddress` with signature `MintToZeroAddress()` and selector `[46, 7, 99, 0]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthError,
        ethers :: contract :: EthDisplay,
    )]
    #[etherror(name = "MintToZeroAddress", abi = "MintToZeroAddress()")]
    pub struct MintToZeroAddress;
    #[doc = "Custom Error type `MintZeroQuantity` with signature `MintZeroQuantity()` and selector `[181, 98, 232, 221]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthError,
        ethers :: contract :: EthDisplay,
    )]
    #[etherror(name = "MintZeroQuantity", abi = "MintZeroQuantity()")]
    pub struct MintZeroQuantity;
    #[doc = "Custom Error type `OwnerQueryForNonexistentToken` with signature `OwnerQueryForNonexistentToken()` and selector `[223, 45, 155, 66]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthError,
        ethers :: contract :: EthDisplay,
    )]
    #[etherror(
        name = "OwnerQueryForNonexistentToken",
        abi = "OwnerQueryForNonexistentToken()"
    )]
    pub struct OwnerQueryForNonexistentToken;
    #[doc = "Custom Error type `OwnershipNotInitializedForExtraData` with signature `OwnershipNotInitializedForExtraData()` and selector `[0, 213, 129, 83]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthError,
        ethers :: contract :: EthDisplay,
    )]
    #[etherror(
        name = "OwnershipNotInitializedForExtraData",
        abi = "OwnershipNotInitializedForExtraData()"
    )]
    pub struct OwnershipNotInitializedForExtraData;
    #[doc = "Custom Error type `TransferCallerNotOwnerNorApproved` with signature `TransferCallerNotOwnerNorApproved()` and selector `[89, 200, 150, 190]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthError,
        ethers :: contract :: EthDisplay,
    )]
    #[etherror(
        name = "TransferCallerNotOwnerNorApproved",
        abi = "TransferCallerNotOwnerNorApproved()"
    )]
    pub struct TransferCallerNotOwnerNorApproved;
    #[doc = "Custom Error type `TransferFromIncorrectOwner` with signature `TransferFromIncorrectOwner()` and selector `[161, 20, 129, 0]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthError,
        ethers :: contract :: EthDisplay,
    )]
    #[etherror(
        name = "TransferFromIncorrectOwner",
        abi = "TransferFromIncorrectOwner()"
    )]
    pub struct TransferFromIncorrectOwner;
    #[doc = "Custom Error type `TransferToNonERC721ReceiverImplementer` with signature `TransferToNonERC721ReceiverImplementer()` and selector `[209, 165, 126, 214]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthError,
        ethers :: contract :: EthDisplay,
    )]
    #[etherror(
        name = "TransferToNonERC721ReceiverImplementer",
        abi = "TransferToNonERC721ReceiverImplementer()"
    )]
    pub struct TransferToNonERC721ReceiverImplementer;
    #[doc = "Custom Error type `TransferToZeroAddress` with signature `TransferToZeroAddress()` and selector `[234, 85, 59, 52]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthError,
        ethers :: contract :: EthDisplay,
    )]
    #[etherror(name = "TransferToZeroAddress", abi = "TransferToZeroAddress()")]
    pub struct TransferToZeroAddress;
    #[doc = "Custom Error type `URIQueryForNonexistentToken` with signature `URIQueryForNonexistentToken()` and selector `[161, 76, 75, 80]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthError,
        ethers :: contract :: EthDisplay,
    )]
    #[etherror(
        name = "URIQueryForNonexistentToken",
        abi = "URIQueryForNonexistentToken()"
    )]
    pub struct URIQueryForNonexistentToken;
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum ERC721AErrors {
        ApprovalCallerNotOwnerNorApproved(ApprovalCallerNotOwnerNorApproved),
        ApprovalQueryForNonexistentToken(ApprovalQueryForNonexistentToken),
        ApproveToCaller(ApproveToCaller),
        BalanceQueryForZeroAddress(BalanceQueryForZeroAddress),
        MintERC2309QuantityExceedsLimit(MintERC2309QuantityExceedsLimit),
        MintToZeroAddress(MintToZeroAddress),
        MintZeroQuantity(MintZeroQuantity),
        OwnerQueryForNonexistentToken(OwnerQueryForNonexistentToken),
        OwnershipNotInitializedForExtraData(OwnershipNotInitializedForExtraData),
        TransferCallerNotOwnerNorApproved(TransferCallerNotOwnerNorApproved),
        TransferFromIncorrectOwner(TransferFromIncorrectOwner),
        TransferToNonERC721ReceiverImplementer(TransferToNonERC721ReceiverImplementer),
        TransferToZeroAddress(TransferToZeroAddress),
        URIQueryForNonexistentToken(URIQueryForNonexistentToken),
    }
    impl ethers::core::abi::AbiDecode for ERC721AErrors {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::std::result::Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <ApprovalCallerNotOwnerNorApproved as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(ERC721AErrors::ApprovalCallerNotOwnerNorApproved(decoded));
            }
            if let Ok(decoded) =
                <ApprovalQueryForNonexistentToken as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(ERC721AErrors::ApprovalQueryForNonexistentToken(decoded));
            }
            if let Ok(decoded) =
                <ApproveToCaller as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ERC721AErrors::ApproveToCaller(decoded));
            }
            if let Ok(decoded) =
                <BalanceQueryForZeroAddress as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ERC721AErrors::BalanceQueryForZeroAddress(decoded));
            }
            if let Ok(decoded) =
                <MintERC2309QuantityExceedsLimit as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(ERC721AErrors::MintERC2309QuantityExceedsLimit(decoded));
            }
            if let Ok(decoded) =
                <MintToZeroAddress as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ERC721AErrors::MintToZeroAddress(decoded));
            }
            if let Ok(decoded) =
                <MintZeroQuantity as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ERC721AErrors::MintZeroQuantity(decoded));
            }
            if let Ok(decoded) =
                <OwnerQueryForNonexistentToken as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(ERC721AErrors::OwnerQueryForNonexistentToken(decoded));
            }
            if let Ok(decoded) =
                <OwnershipNotInitializedForExtraData as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(ERC721AErrors::OwnershipNotInitializedForExtraData(decoded));
            }
            if let Ok(decoded) =
                <TransferCallerNotOwnerNorApproved as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(ERC721AErrors::TransferCallerNotOwnerNorApproved(decoded));
            }
            if let Ok(decoded) =
                <TransferFromIncorrectOwner as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ERC721AErrors::TransferFromIncorrectOwner(decoded));
            }
            if let Ok(decoded) =
                <TransferToNonERC721ReceiverImplementer as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(ERC721AErrors::TransferToNonERC721ReceiverImplementer(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <TransferToZeroAddress as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ERC721AErrors::TransferToZeroAddress(decoded));
            }
            if let Ok(decoded) =
                <URIQueryForNonexistentToken as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ERC721AErrors::URIQueryForNonexistentToken(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for ERC721AErrors {
        fn encode(self) -> Vec<u8> {
            match self {
                ERC721AErrors::ApprovalCallerNotOwnerNorApproved(element) => element.encode(),
                ERC721AErrors::ApprovalQueryForNonexistentToken(element) => element.encode(),
                ERC721AErrors::ApproveToCaller(element) => element.encode(),
                ERC721AErrors::BalanceQueryForZeroAddress(element) => element.encode(),
                ERC721AErrors::MintERC2309QuantityExceedsLimit(element) => element.encode(),
                ERC721AErrors::MintToZeroAddress(element) => element.encode(),
                ERC721AErrors::MintZeroQuantity(element) => element.encode(),
                ERC721AErrors::OwnerQueryForNonexistentToken(element) => element.encode(),
                ERC721AErrors::OwnershipNotInitializedForExtraData(element) => element.encode(),
                ERC721AErrors::TransferCallerNotOwnerNorApproved(element) => element.encode(),
                ERC721AErrors::TransferFromIncorrectOwner(element) => element.encode(),
                ERC721AErrors::TransferToNonERC721ReceiverImplementer(element) => element.encode(),
                ERC721AErrors::TransferToZeroAddress(element) => element.encode(),
                ERC721AErrors::URIQueryForNonexistentToken(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for ERC721AErrors {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                ERC721AErrors::ApprovalCallerNotOwnerNorApproved(element) => element.fmt(f),
                ERC721AErrors::ApprovalQueryForNonexistentToken(element) => element.fmt(f),
                ERC721AErrors::ApproveToCaller(element) => element.fmt(f),
                ERC721AErrors::BalanceQueryForZeroAddress(element) => element.fmt(f),
                ERC721AErrors::MintERC2309QuantityExceedsLimit(element) => element.fmt(f),
                ERC721AErrors::MintToZeroAddress(element) => element.fmt(f),
                ERC721AErrors::MintZeroQuantity(element) => element.fmt(f),
                ERC721AErrors::OwnerQueryForNonexistentToken(element) => element.fmt(f),
                ERC721AErrors::OwnershipNotInitializedForExtraData(element) => element.fmt(f),
                ERC721AErrors::TransferCallerNotOwnerNorApproved(element) => element.fmt(f),
                ERC721AErrors::TransferFromIncorrectOwner(element) => element.fmt(f),
                ERC721AErrors::TransferToNonERC721ReceiverImplementer(element) => element.fmt(f),
                ERC721AErrors::TransferToZeroAddress(element) => element.fmt(f),
                ERC721AErrors::URIQueryForNonexistentToken(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<ApprovalCallerNotOwnerNorApproved> for ERC721AErrors {
        fn from(var: ApprovalCallerNotOwnerNorApproved) -> Self {
            ERC721AErrors::ApprovalCallerNotOwnerNorApproved(var)
        }
    }
    impl ::std::convert::From<ApprovalQueryForNonexistentToken> for ERC721AErrors {
        fn from(var: ApprovalQueryForNonexistentToken) -> Self {
            ERC721AErrors::ApprovalQueryForNonexistentToken(var)
        }
    }
    impl ::std::convert::From<ApproveToCaller> for ERC721AErrors {
        fn from(var: ApproveToCaller) -> Self {
            ERC721AErrors::ApproveToCaller(var)
        }
    }
    impl ::std::convert::From<BalanceQueryForZeroAddress> for ERC721AErrors {
        fn from(var: BalanceQueryForZeroAddress) -> Self {
            ERC721AErrors::BalanceQueryForZeroAddress(var)
        }
    }
    impl ::std::convert::From<MintERC2309QuantityExceedsLimit> for ERC721AErrors {
        fn from(var: MintERC2309QuantityExceedsLimit) -> Self {
            ERC721AErrors::MintERC2309QuantityExceedsLimit(var)
        }
    }
    impl ::std::convert::From<MintToZeroAddress> for ERC721AErrors {
        fn from(var: MintToZeroAddress) -> Self {
            ERC721AErrors::MintToZeroAddress(var)
        }
    }
    impl ::std::convert::From<MintZeroQuantity> for ERC721AErrors {
        fn from(var: MintZeroQuantity) -> Self {
            ERC721AErrors::MintZeroQuantity(var)
        }
    }
    impl ::std::convert::From<OwnerQueryForNonexistentToken> for ERC721AErrors {
        fn from(var: OwnerQueryForNonexistentToken) -> Self {
            ERC721AErrors::OwnerQueryForNonexistentToken(var)
        }
    }
    impl ::std::convert::From<OwnershipNotInitializedForExtraData> for ERC721AErrors {
        fn from(var: OwnershipNotInitializedForExtraData) -> Self {
            ERC721AErrors::OwnershipNotInitializedForExtraData(var)
        }
    }
    impl ::std::convert::From<TransferCallerNotOwnerNorApproved> for ERC721AErrors {
        fn from(var: TransferCallerNotOwnerNorApproved) -> Self {
            ERC721AErrors::TransferCallerNotOwnerNorApproved(var)
        }
    }
    impl ::std::convert::From<TransferFromIncorrectOwner> for ERC721AErrors {
        fn from(var: TransferFromIncorrectOwner) -> Self {
            ERC721AErrors::TransferFromIncorrectOwner(var)
        }
    }
    impl ::std::convert::From<TransferToNonERC721ReceiverImplementer> for ERC721AErrors {
        fn from(var: TransferToNonERC721ReceiverImplementer) -> Self {
            ERC721AErrors::TransferToNonERC721ReceiverImplementer(var)
        }
    }
    impl ::std::convert::From<TransferToZeroAddress> for ERC721AErrors {
        fn from(var: TransferToZeroAddress) -> Self {
            ERC721AErrors::TransferToZeroAddress(var)
        }
    }
    impl ::std::convert::From<URIQueryForNonexistentToken> for ERC721AErrors {
        fn from(var: URIQueryForNonexistentToken) -> Self {
            ERC721AErrors::URIQueryForNonexistentToken(var)
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
    #[ethevent(name = "Approval", abi = "Approval(address,address,uint256)")]
    pub struct ApprovalFilter {
        #[ethevent(indexed)]
        pub owner: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub approved: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub token_id: ethers::core::types::U256,
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
    #[ethevent(name = "ApprovalForAll", abi = "ApprovalForAll(address,address,bool)")]
    pub struct ApprovalForAllFilter {
        #[ethevent(indexed)]
        pub owner: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub operator: ethers::core::types::Address,
        pub approved: bool,
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
        name = "ConsecutiveTransfer",
        abi = "ConsecutiveTransfer(uint256,uint256,address,address)"
    )]
    pub struct ConsecutiveTransferFilter {
        #[ethevent(indexed)]
        pub from_token_id: ethers::core::types::U256,
        pub to_token_id: ethers::core::types::U256,
        #[ethevent(indexed)]
        pub from: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub to: ethers::core::types::Address,
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
    #[ethevent(name = "Transfer", abi = "Transfer(address,address,uint256)")]
    pub struct TransferFilter {
        #[ethevent(indexed)]
        pub from: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub to: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub token_id: ethers::core::types::U256,
    }
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum ERC721AEvents {
        ApprovalFilter(ApprovalFilter),
        ApprovalForAllFilter(ApprovalForAllFilter),
        ConsecutiveTransferFilter(ConsecutiveTransferFilter),
        TransferFilter(TransferFilter),
    }
    impl ethers::contract::EthLogDecode for ERC721AEvents {
        fn decode_log(
            log: &ethers::core::abi::RawLog,
        ) -> ::std::result::Result<Self, ethers::core::abi::Error>
        where
            Self: Sized,
        {
            if let Ok(decoded) = ApprovalFilter::decode_log(log) {
                return Ok(ERC721AEvents::ApprovalFilter(decoded));
            }
            if let Ok(decoded) = ApprovalForAllFilter::decode_log(log) {
                return Ok(ERC721AEvents::ApprovalForAllFilter(decoded));
            }
            if let Ok(decoded) = ConsecutiveTransferFilter::decode_log(log) {
                return Ok(ERC721AEvents::ConsecutiveTransferFilter(decoded));
            }
            if let Ok(decoded) = TransferFilter::decode_log(log) {
                return Ok(ERC721AEvents::TransferFilter(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::std::fmt::Display for ERC721AEvents {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                ERC721AEvents::ApprovalFilter(element) => element.fmt(f),
                ERC721AEvents::ApprovalForAllFilter(element) => element.fmt(f),
                ERC721AEvents::ConsecutiveTransferFilter(element) => element.fmt(f),
                ERC721AEvents::TransferFilter(element) => element.fmt(f),
            }
        }
    }
    #[doc = "Container type for all input parameters for the `approve` function with signature `approve(address,uint256)` and selector `[9, 94, 167, 179]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "approve", abi = "approve(address,uint256)")]
    pub struct ApproveCall {
        pub to: ethers::core::types::Address,
        pub token_id: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `balanceOf` function with signature `balanceOf(address)` and selector `[112, 160, 130, 49]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "balanceOf", abi = "balanceOf(address)")]
    pub struct BalanceOfCall {
        pub owner: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `getApproved` function with signature `getApproved(uint256)` and selector `[8, 24, 18, 252]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "getApproved", abi = "getApproved(uint256)")]
    pub struct GetApprovedCall {
        pub token_id: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `isApprovedForAll` function with signature `isApprovedForAll(address,address)` and selector `[233, 133, 233, 197]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "isApprovedForAll", abi = "isApprovedForAll(address,address)")]
    pub struct IsApprovedForAllCall {
        pub owner: ethers::core::types::Address,
        pub operator: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `name` function with signature `name()` and selector `[6, 253, 222, 3]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "name", abi = "name()")]
    pub struct NameCall;
    #[doc = "Container type for all input parameters for the `ownerOf` function with signature `ownerOf(uint256)` and selector `[99, 82, 33, 30]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "ownerOf", abi = "ownerOf(uint256)")]
    pub struct OwnerOfCall {
        pub token_id: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `safeTransferFrom` function with signature `safeTransferFrom(address,address,uint256)` and selector `[66, 132, 46, 14]`"]
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
        name = "safeTransferFrom",
        abi = "safeTransferFrom(address,address,uint256)"
    )]
    pub struct SafeTransferFromCall {
        pub from: ethers::core::types::Address,
        pub to: ethers::core::types::Address,
        pub token_id: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `safeTransferFrom` function with signature `safeTransferFrom(address,address,uint256,bytes)` and selector `[184, 141, 79, 222]`"]
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
        name = "safeTransferFrom",
        abi = "safeTransferFrom(address,address,uint256,bytes)"
    )]
    pub struct SafeTransferFromWithFromAndToAndDataCall {
        pub from: ethers::core::types::Address,
        pub to: ethers::core::types::Address,
        pub token_id: ethers::core::types::U256,
        pub data: ethers::core::types::Bytes,
    }
    #[doc = "Container type for all input parameters for the `setApprovalForAll` function with signature `setApprovalForAll(address,bool)` and selector `[162, 44, 180, 101]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "setApprovalForAll", abi = "setApprovalForAll(address,bool)")]
    pub struct SetApprovalForAllCall {
        pub operator: ethers::core::types::Address,
        pub approved: bool,
    }
    #[doc = "Container type for all input parameters for the `supportsInterface` function with signature `supportsInterface(bytes4)` and selector `[1, 255, 201, 167]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "supportsInterface", abi = "supportsInterface(bytes4)")]
    pub struct SupportsInterfaceCall {
        pub interface_id: [u8; 4],
    }
    #[doc = "Container type for all input parameters for the `symbol` function with signature `symbol()` and selector `[149, 216, 155, 65]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "symbol", abi = "symbol()")]
    pub struct SymbolCall;
    #[doc = "Container type for all input parameters for the `tokenURI` function with signature `tokenURI(uint256)` and selector `[200, 123, 86, 221]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "tokenURI", abi = "tokenURI(uint256)")]
    pub struct TokenURICall {
        pub token_id: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `totalSupply` function with signature `totalSupply()` and selector `[24, 22, 13, 221]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "totalSupply", abi = "totalSupply()")]
    pub struct TotalSupplyCall;
    #[doc = "Container type for all input parameters for the `transferFrom` function with signature `transferFrom(address,address,uint256)` and selector `[35, 184, 114, 221]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "transferFrom", abi = "transferFrom(address,address,uint256)")]
    pub struct TransferFromCall {
        pub from: ethers::core::types::Address,
        pub to: ethers::core::types::Address,
        pub token_id: ethers::core::types::U256,
    }
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum ERC721ACalls {
        Approve(ApproveCall),
        BalanceOf(BalanceOfCall),
        GetApproved(GetApprovedCall),
        IsApprovedForAll(IsApprovedForAllCall),
        Name(NameCall),
        OwnerOf(OwnerOfCall),
        SafeTransferFrom(SafeTransferFromCall),
        SafeTransferFromWithFromAndToAndData(SafeTransferFromWithFromAndToAndDataCall),
        SetApprovalForAll(SetApprovalForAllCall),
        SupportsInterface(SupportsInterfaceCall),
        Symbol(SymbolCall),
        TokenURI(TokenURICall),
        TotalSupply(TotalSupplyCall),
        TransferFrom(TransferFromCall),
    }
    impl ethers::core::abi::AbiDecode for ERC721ACalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::std::result::Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <ApproveCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ERC721ACalls::Approve(decoded));
            }
            if let Ok(decoded) =
                <BalanceOfCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ERC721ACalls::BalanceOf(decoded));
            }
            if let Ok(decoded) =
                <GetApprovedCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ERC721ACalls::GetApproved(decoded));
            }
            if let Ok(decoded) =
                <IsApprovedForAllCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ERC721ACalls::IsApprovedForAll(decoded));
            }
            if let Ok(decoded) = <NameCall as ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(ERC721ACalls::Name(decoded));
            }
            if let Ok(decoded) =
                <OwnerOfCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ERC721ACalls::OwnerOf(decoded));
            }
            if let Ok(decoded) =
                <SafeTransferFromCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ERC721ACalls::SafeTransferFrom(decoded));
            }
            if let Ok(decoded) =
                <SafeTransferFromWithFromAndToAndDataCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(ERC721ACalls::SafeTransferFromWithFromAndToAndData(decoded));
            }
            if let Ok(decoded) =
                <SetApprovalForAllCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ERC721ACalls::SetApprovalForAll(decoded));
            }
            if let Ok(decoded) =
                <SupportsInterfaceCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ERC721ACalls::SupportsInterface(decoded));
            }
            if let Ok(decoded) = <SymbolCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ERC721ACalls::Symbol(decoded));
            }
            if let Ok(decoded) =
                <TokenURICall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ERC721ACalls::TokenURI(decoded));
            }
            if let Ok(decoded) =
                <TotalSupplyCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ERC721ACalls::TotalSupply(decoded));
            }
            if let Ok(decoded) =
                <TransferFromCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ERC721ACalls::TransferFrom(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for ERC721ACalls {
        fn encode(self) -> Vec<u8> {
            match self {
                ERC721ACalls::Approve(element) => element.encode(),
                ERC721ACalls::BalanceOf(element) => element.encode(),
                ERC721ACalls::GetApproved(element) => element.encode(),
                ERC721ACalls::IsApprovedForAll(element) => element.encode(),
                ERC721ACalls::Name(element) => element.encode(),
                ERC721ACalls::OwnerOf(element) => element.encode(),
                ERC721ACalls::SafeTransferFrom(element) => element.encode(),
                ERC721ACalls::SafeTransferFromWithFromAndToAndData(element) => element.encode(),
                ERC721ACalls::SetApprovalForAll(element) => element.encode(),
                ERC721ACalls::SupportsInterface(element) => element.encode(),
                ERC721ACalls::Symbol(element) => element.encode(),
                ERC721ACalls::TokenURI(element) => element.encode(),
                ERC721ACalls::TotalSupply(element) => element.encode(),
                ERC721ACalls::TransferFrom(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for ERC721ACalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                ERC721ACalls::Approve(element) => element.fmt(f),
                ERC721ACalls::BalanceOf(element) => element.fmt(f),
                ERC721ACalls::GetApproved(element) => element.fmt(f),
                ERC721ACalls::IsApprovedForAll(element) => element.fmt(f),
                ERC721ACalls::Name(element) => element.fmt(f),
                ERC721ACalls::OwnerOf(element) => element.fmt(f),
                ERC721ACalls::SafeTransferFrom(element) => element.fmt(f),
                ERC721ACalls::SafeTransferFromWithFromAndToAndData(element) => element.fmt(f),
                ERC721ACalls::SetApprovalForAll(element) => element.fmt(f),
                ERC721ACalls::SupportsInterface(element) => element.fmt(f),
                ERC721ACalls::Symbol(element) => element.fmt(f),
                ERC721ACalls::TokenURI(element) => element.fmt(f),
                ERC721ACalls::TotalSupply(element) => element.fmt(f),
                ERC721ACalls::TransferFrom(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<ApproveCall> for ERC721ACalls {
        fn from(var: ApproveCall) -> Self {
            ERC721ACalls::Approve(var)
        }
    }
    impl ::std::convert::From<BalanceOfCall> for ERC721ACalls {
        fn from(var: BalanceOfCall) -> Self {
            ERC721ACalls::BalanceOf(var)
        }
    }
    impl ::std::convert::From<GetApprovedCall> for ERC721ACalls {
        fn from(var: GetApprovedCall) -> Self {
            ERC721ACalls::GetApproved(var)
        }
    }
    impl ::std::convert::From<IsApprovedForAllCall> for ERC721ACalls {
        fn from(var: IsApprovedForAllCall) -> Self {
            ERC721ACalls::IsApprovedForAll(var)
        }
    }
    impl ::std::convert::From<NameCall> for ERC721ACalls {
        fn from(var: NameCall) -> Self {
            ERC721ACalls::Name(var)
        }
    }
    impl ::std::convert::From<OwnerOfCall> for ERC721ACalls {
        fn from(var: OwnerOfCall) -> Self {
            ERC721ACalls::OwnerOf(var)
        }
    }
    impl ::std::convert::From<SafeTransferFromCall> for ERC721ACalls {
        fn from(var: SafeTransferFromCall) -> Self {
            ERC721ACalls::SafeTransferFrom(var)
        }
    }
    impl ::std::convert::From<SafeTransferFromWithFromAndToAndDataCall> for ERC721ACalls {
        fn from(var: SafeTransferFromWithFromAndToAndDataCall) -> Self {
            ERC721ACalls::SafeTransferFromWithFromAndToAndData(var)
        }
    }
    impl ::std::convert::From<SetApprovalForAllCall> for ERC721ACalls {
        fn from(var: SetApprovalForAllCall) -> Self {
            ERC721ACalls::SetApprovalForAll(var)
        }
    }
    impl ::std::convert::From<SupportsInterfaceCall> for ERC721ACalls {
        fn from(var: SupportsInterfaceCall) -> Self {
            ERC721ACalls::SupportsInterface(var)
        }
    }
    impl ::std::convert::From<SymbolCall> for ERC721ACalls {
        fn from(var: SymbolCall) -> Self {
            ERC721ACalls::Symbol(var)
        }
    }
    impl ::std::convert::From<TokenURICall> for ERC721ACalls {
        fn from(var: TokenURICall) -> Self {
            ERC721ACalls::TokenURI(var)
        }
    }
    impl ::std::convert::From<TotalSupplyCall> for ERC721ACalls {
        fn from(var: TotalSupplyCall) -> Self {
            ERC721ACalls::TotalSupply(var)
        }
    }
    impl ::std::convert::From<TransferFromCall> for ERC721ACalls {
        fn from(var: TransferFromCall) -> Self {
            ERC721ACalls::TransferFrom(var)
        }
    }
    #[doc = "Container type for all return fields from the `balanceOf` function with signature `balanceOf(address)` and selector `[112, 160, 130, 49]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct BalanceOfReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `getApproved` function with signature `getApproved(uint256)` and selector `[8, 24, 18, 252]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct GetApprovedReturn(pub ethers::core::types::Address);
    #[doc = "Container type for all return fields from the `isApprovedForAll` function with signature `isApprovedForAll(address,address)` and selector `[233, 133, 233, 197]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct IsApprovedForAllReturn(pub bool);
    #[doc = "Container type for all return fields from the `name` function with signature `name()` and selector `[6, 253, 222, 3]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct NameReturn(pub String);
    #[doc = "Container type for all return fields from the `ownerOf` function with signature `ownerOf(uint256)` and selector `[99, 82, 33, 30]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct OwnerOfReturn(pub ethers::core::types::Address);
    #[doc = "Container type for all return fields from the `supportsInterface` function with signature `supportsInterface(bytes4)` and selector `[1, 255, 201, 167]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct SupportsInterfaceReturn(pub bool);
    #[doc = "Container type for all return fields from the `symbol` function with signature `symbol()` and selector `[149, 216, 155, 65]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct SymbolReturn(pub String);
    #[doc = "Container type for all return fields from the `tokenURI` function with signature `tokenURI(uint256)` and selector `[200, 123, 86, 221]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct TokenURIReturn(pub String);
    #[doc = "Container type for all return fields from the `totalSupply` function with signature `totalSupply()` and selector `[24, 22, 13, 221]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct TotalSupplyReturn(pub ethers::core::types::U256);
}
