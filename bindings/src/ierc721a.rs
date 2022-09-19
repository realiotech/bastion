pub use ierc721a::*;
#[allow(clippy::too_many_arguments, non_camel_case_types)]
pub mod ierc721a {
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
    #[doc = "IERC721A was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static IERC721A_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            ethers :: core :: utils :: __serde_json :: from_str ("[{\"inputs\":[],\"type\":\"error\",\"name\":\"ApprovalCallerNotOwnerNorApproved\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"ApprovalQueryForNonexistentToken\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"ApproveToCaller\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"BalanceQueryForZeroAddress\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"MintERC2309QuantityExceedsLimit\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"MintToZeroAddress\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"MintZeroQuantity\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"OwnerQueryForNonexistentToken\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"OwnershipNotInitializedForExtraData\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"TransferCallerNotOwnerNorApproved\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"TransferFromIncorrectOwner\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"TransferToNonERC721ReceiverImplementer\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"TransferToZeroAddress\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"URIQueryForNonexistentToken\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"owner\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"approved\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"tokenId\",\"type\":\"uint256\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"Approval\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"owner\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"operator\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"bool\",\"name\":\"approved\",\"type\":\"bool\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"ApprovalForAll\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"fromTokenId\",\"type\":\"uint256\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"toTokenId\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"address\",\"name\":\"from\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"to\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"ConsecutiveTransfer\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"from\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"to\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"tokenId\",\"type\":\"uint256\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"Transfer\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"to\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"tokenId\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"approve\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"owner\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"balanceOf\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"balance\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"tokenId\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getApproved\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"operator\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"owner\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"operator\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"isApprovedForAll\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"name\",\"outputs\":[{\"internalType\":\"string\",\"name\":\"\",\"type\":\"string\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"tokenId\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"ownerOf\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"owner\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"from\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"to\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"tokenId\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"safeTransferFrom\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"from\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"to\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"tokenId\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"data\",\"type\":\"bytes\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"safeTransferFrom\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"operator\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bool\",\"name\":\"_approved\",\"type\":\"bool\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setApprovalForAll\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"bytes4\",\"name\":\"interfaceId\",\"type\":\"bytes4\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"supportsInterface\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"symbol\",\"outputs\":[{\"internalType\":\"string\",\"name\":\"\",\"type\":\"string\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"tokenId\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"tokenURI\",\"outputs\":[{\"internalType\":\"string\",\"name\":\"\",\"type\":\"string\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"totalSupply\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"from\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"to\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"tokenId\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"transferFrom\",\"outputs\":[]}]") . expect ("invalid abi")
        });
    pub struct IERC721A<M>(ethers::contract::Contract<M>);
    impl<M> Clone for IERC721A<M> {
        fn clone(&self) -> Self {
            IERC721A(self.0.clone())
        }
    }
    impl<M> std::ops::Deref for IERC721A<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers::providers::Middleware> std::fmt::Debug for IERC721A<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(IERC721A))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ethers::providers::Middleware> IERC721A<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(address.into(), IERC721A_ABI.clone(), client).into()
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
        pub fn safe_transfer_from_with_data(
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
        pub fn events(&self) -> ethers::contract::builders::Event<M, IERC721AEvents> {
            self.0.event_with_filter(Default::default())
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>> for IERC721A<M> {
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
    pub enum IERC721AErrors {
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
    impl ethers::core::abi::AbiDecode for IERC721AErrors {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::std::result::Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <ApprovalCallerNotOwnerNorApproved as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(IERC721AErrors::ApprovalCallerNotOwnerNorApproved(decoded));
            }
            if let Ok(decoded) =
                <ApprovalQueryForNonexistentToken as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(IERC721AErrors::ApprovalQueryForNonexistentToken(decoded));
            }
            if let Ok(decoded) =
                <ApproveToCaller as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IERC721AErrors::ApproveToCaller(decoded));
            }
            if let Ok(decoded) =
                <BalanceQueryForZeroAddress as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IERC721AErrors::BalanceQueryForZeroAddress(decoded));
            }
            if let Ok(decoded) =
                <MintERC2309QuantityExceedsLimit as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(IERC721AErrors::MintERC2309QuantityExceedsLimit(decoded));
            }
            if let Ok(decoded) =
                <MintToZeroAddress as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IERC721AErrors::MintToZeroAddress(decoded));
            }
            if let Ok(decoded) =
                <MintZeroQuantity as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IERC721AErrors::MintZeroQuantity(decoded));
            }
            if let Ok(decoded) =
                <OwnerQueryForNonexistentToken as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(IERC721AErrors::OwnerQueryForNonexistentToken(decoded));
            }
            if let Ok(decoded) =
                <OwnershipNotInitializedForExtraData as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(IERC721AErrors::OwnershipNotInitializedForExtraData(decoded));
            }
            if let Ok(decoded) =
                <TransferCallerNotOwnerNorApproved as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(IERC721AErrors::TransferCallerNotOwnerNorApproved(decoded));
            }
            if let Ok(decoded) =
                <TransferFromIncorrectOwner as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IERC721AErrors::TransferFromIncorrectOwner(decoded));
            }
            if let Ok(decoded) =
                <TransferToNonERC721ReceiverImplementer as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(IERC721AErrors::TransferToNonERC721ReceiverImplementer(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <TransferToZeroAddress as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IERC721AErrors::TransferToZeroAddress(decoded));
            }
            if let Ok(decoded) =
                <URIQueryForNonexistentToken as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IERC721AErrors::URIQueryForNonexistentToken(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for IERC721AErrors {
        fn encode(self) -> Vec<u8> {
            match self {
                IERC721AErrors::ApprovalCallerNotOwnerNorApproved(element) => element.encode(),
                IERC721AErrors::ApprovalQueryForNonexistentToken(element) => element.encode(),
                IERC721AErrors::ApproveToCaller(element) => element.encode(),
                IERC721AErrors::BalanceQueryForZeroAddress(element) => element.encode(),
                IERC721AErrors::MintERC2309QuantityExceedsLimit(element) => element.encode(),
                IERC721AErrors::MintToZeroAddress(element) => element.encode(),
                IERC721AErrors::MintZeroQuantity(element) => element.encode(),
                IERC721AErrors::OwnerQueryForNonexistentToken(element) => element.encode(),
                IERC721AErrors::OwnershipNotInitializedForExtraData(element) => element.encode(),
                IERC721AErrors::TransferCallerNotOwnerNorApproved(element) => element.encode(),
                IERC721AErrors::TransferFromIncorrectOwner(element) => element.encode(),
                IERC721AErrors::TransferToNonERC721ReceiverImplementer(element) => element.encode(),
                IERC721AErrors::TransferToZeroAddress(element) => element.encode(),
                IERC721AErrors::URIQueryForNonexistentToken(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for IERC721AErrors {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                IERC721AErrors::ApprovalCallerNotOwnerNorApproved(element) => element.fmt(f),
                IERC721AErrors::ApprovalQueryForNonexistentToken(element) => element.fmt(f),
                IERC721AErrors::ApproveToCaller(element) => element.fmt(f),
                IERC721AErrors::BalanceQueryForZeroAddress(element) => element.fmt(f),
                IERC721AErrors::MintERC2309QuantityExceedsLimit(element) => element.fmt(f),
                IERC721AErrors::MintToZeroAddress(element) => element.fmt(f),
                IERC721AErrors::MintZeroQuantity(element) => element.fmt(f),
                IERC721AErrors::OwnerQueryForNonexistentToken(element) => element.fmt(f),
                IERC721AErrors::OwnershipNotInitializedForExtraData(element) => element.fmt(f),
                IERC721AErrors::TransferCallerNotOwnerNorApproved(element) => element.fmt(f),
                IERC721AErrors::TransferFromIncorrectOwner(element) => element.fmt(f),
                IERC721AErrors::TransferToNonERC721ReceiverImplementer(element) => element.fmt(f),
                IERC721AErrors::TransferToZeroAddress(element) => element.fmt(f),
                IERC721AErrors::URIQueryForNonexistentToken(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<ApprovalCallerNotOwnerNorApproved> for IERC721AErrors {
        fn from(var: ApprovalCallerNotOwnerNorApproved) -> Self {
            IERC721AErrors::ApprovalCallerNotOwnerNorApproved(var)
        }
    }
    impl ::std::convert::From<ApprovalQueryForNonexistentToken> for IERC721AErrors {
        fn from(var: ApprovalQueryForNonexistentToken) -> Self {
            IERC721AErrors::ApprovalQueryForNonexistentToken(var)
        }
    }
    impl ::std::convert::From<ApproveToCaller> for IERC721AErrors {
        fn from(var: ApproveToCaller) -> Self {
            IERC721AErrors::ApproveToCaller(var)
        }
    }
    impl ::std::convert::From<BalanceQueryForZeroAddress> for IERC721AErrors {
        fn from(var: BalanceQueryForZeroAddress) -> Self {
            IERC721AErrors::BalanceQueryForZeroAddress(var)
        }
    }
    impl ::std::convert::From<MintERC2309QuantityExceedsLimit> for IERC721AErrors {
        fn from(var: MintERC2309QuantityExceedsLimit) -> Self {
            IERC721AErrors::MintERC2309QuantityExceedsLimit(var)
        }
    }
    impl ::std::convert::From<MintToZeroAddress> for IERC721AErrors {
        fn from(var: MintToZeroAddress) -> Self {
            IERC721AErrors::MintToZeroAddress(var)
        }
    }
    impl ::std::convert::From<MintZeroQuantity> for IERC721AErrors {
        fn from(var: MintZeroQuantity) -> Self {
            IERC721AErrors::MintZeroQuantity(var)
        }
    }
    impl ::std::convert::From<OwnerQueryForNonexistentToken> for IERC721AErrors {
        fn from(var: OwnerQueryForNonexistentToken) -> Self {
            IERC721AErrors::OwnerQueryForNonexistentToken(var)
        }
    }
    impl ::std::convert::From<OwnershipNotInitializedForExtraData> for IERC721AErrors {
        fn from(var: OwnershipNotInitializedForExtraData) -> Self {
            IERC721AErrors::OwnershipNotInitializedForExtraData(var)
        }
    }
    impl ::std::convert::From<TransferCallerNotOwnerNorApproved> for IERC721AErrors {
        fn from(var: TransferCallerNotOwnerNorApproved) -> Self {
            IERC721AErrors::TransferCallerNotOwnerNorApproved(var)
        }
    }
    impl ::std::convert::From<TransferFromIncorrectOwner> for IERC721AErrors {
        fn from(var: TransferFromIncorrectOwner) -> Self {
            IERC721AErrors::TransferFromIncorrectOwner(var)
        }
    }
    impl ::std::convert::From<TransferToNonERC721ReceiverImplementer> for IERC721AErrors {
        fn from(var: TransferToNonERC721ReceiverImplementer) -> Self {
            IERC721AErrors::TransferToNonERC721ReceiverImplementer(var)
        }
    }
    impl ::std::convert::From<TransferToZeroAddress> for IERC721AErrors {
        fn from(var: TransferToZeroAddress) -> Self {
            IERC721AErrors::TransferToZeroAddress(var)
        }
    }
    impl ::std::convert::From<URIQueryForNonexistentToken> for IERC721AErrors {
        fn from(var: URIQueryForNonexistentToken) -> Self {
            IERC721AErrors::URIQueryForNonexistentToken(var)
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
    pub enum IERC721AEvents {
        ApprovalFilter(ApprovalFilter),
        ApprovalForAllFilter(ApprovalForAllFilter),
        ConsecutiveTransferFilter(ConsecutiveTransferFilter),
        TransferFilter(TransferFilter),
    }
    impl ethers::contract::EthLogDecode for IERC721AEvents {
        fn decode_log(
            log: &ethers::core::abi::RawLog,
        ) -> ::std::result::Result<Self, ethers::core::abi::Error>
        where
            Self: Sized,
        {
            if let Ok(decoded) = ApprovalFilter::decode_log(log) {
                return Ok(IERC721AEvents::ApprovalFilter(decoded));
            }
            if let Ok(decoded) = ApprovalForAllFilter::decode_log(log) {
                return Ok(IERC721AEvents::ApprovalForAllFilter(decoded));
            }
            if let Ok(decoded) = ConsecutiveTransferFilter::decode_log(log) {
                return Ok(IERC721AEvents::ConsecutiveTransferFilter(decoded));
            }
            if let Ok(decoded) = TransferFilter::decode_log(log) {
                return Ok(IERC721AEvents::TransferFilter(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::std::fmt::Display for IERC721AEvents {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                IERC721AEvents::ApprovalFilter(element) => element.fmt(f),
                IERC721AEvents::ApprovalForAllFilter(element) => element.fmt(f),
                IERC721AEvents::ConsecutiveTransferFilter(element) => element.fmt(f),
                IERC721AEvents::TransferFilter(element) => element.fmt(f),
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
    pub struct SafeTransferFromWithDataCall {
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
    pub enum IERC721ACalls {
        Approve(ApproveCall),
        BalanceOf(BalanceOfCall),
        GetApproved(GetApprovedCall),
        IsApprovedForAll(IsApprovedForAllCall),
        Name(NameCall),
        OwnerOf(OwnerOfCall),
        SafeTransferFrom(SafeTransferFromCall),
        SafeTransferFromWithData(SafeTransferFromWithDataCall),
        SetApprovalForAll(SetApprovalForAllCall),
        SupportsInterface(SupportsInterfaceCall),
        Symbol(SymbolCall),
        TokenURI(TokenURICall),
        TotalSupply(TotalSupplyCall),
        TransferFrom(TransferFromCall),
    }
    impl ethers::core::abi::AbiDecode for IERC721ACalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::std::result::Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <ApproveCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IERC721ACalls::Approve(decoded));
            }
            if let Ok(decoded) =
                <BalanceOfCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IERC721ACalls::BalanceOf(decoded));
            }
            if let Ok(decoded) =
                <GetApprovedCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IERC721ACalls::GetApproved(decoded));
            }
            if let Ok(decoded) =
                <IsApprovedForAllCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IERC721ACalls::IsApprovedForAll(decoded));
            }
            if let Ok(decoded) = <NameCall as ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(IERC721ACalls::Name(decoded));
            }
            if let Ok(decoded) =
                <OwnerOfCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IERC721ACalls::OwnerOf(decoded));
            }
            if let Ok(decoded) =
                <SafeTransferFromCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IERC721ACalls::SafeTransferFrom(decoded));
            }
            if let Ok(decoded) =
                <SafeTransferFromWithDataCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(IERC721ACalls::SafeTransferFromWithData(decoded));
            }
            if let Ok(decoded) =
                <SetApprovalForAllCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IERC721ACalls::SetApprovalForAll(decoded));
            }
            if let Ok(decoded) =
                <SupportsInterfaceCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IERC721ACalls::SupportsInterface(decoded));
            }
            if let Ok(decoded) = <SymbolCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IERC721ACalls::Symbol(decoded));
            }
            if let Ok(decoded) =
                <TokenURICall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IERC721ACalls::TokenURI(decoded));
            }
            if let Ok(decoded) =
                <TotalSupplyCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IERC721ACalls::TotalSupply(decoded));
            }
            if let Ok(decoded) =
                <TransferFromCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(IERC721ACalls::TransferFrom(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for IERC721ACalls {
        fn encode(self) -> Vec<u8> {
            match self {
                IERC721ACalls::Approve(element) => element.encode(),
                IERC721ACalls::BalanceOf(element) => element.encode(),
                IERC721ACalls::GetApproved(element) => element.encode(),
                IERC721ACalls::IsApprovedForAll(element) => element.encode(),
                IERC721ACalls::Name(element) => element.encode(),
                IERC721ACalls::OwnerOf(element) => element.encode(),
                IERC721ACalls::SafeTransferFrom(element) => element.encode(),
                IERC721ACalls::SafeTransferFromWithData(element) => element.encode(),
                IERC721ACalls::SetApprovalForAll(element) => element.encode(),
                IERC721ACalls::SupportsInterface(element) => element.encode(),
                IERC721ACalls::Symbol(element) => element.encode(),
                IERC721ACalls::TokenURI(element) => element.encode(),
                IERC721ACalls::TotalSupply(element) => element.encode(),
                IERC721ACalls::TransferFrom(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for IERC721ACalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                IERC721ACalls::Approve(element) => element.fmt(f),
                IERC721ACalls::BalanceOf(element) => element.fmt(f),
                IERC721ACalls::GetApproved(element) => element.fmt(f),
                IERC721ACalls::IsApprovedForAll(element) => element.fmt(f),
                IERC721ACalls::Name(element) => element.fmt(f),
                IERC721ACalls::OwnerOf(element) => element.fmt(f),
                IERC721ACalls::SafeTransferFrom(element) => element.fmt(f),
                IERC721ACalls::SafeTransferFromWithData(element) => element.fmt(f),
                IERC721ACalls::SetApprovalForAll(element) => element.fmt(f),
                IERC721ACalls::SupportsInterface(element) => element.fmt(f),
                IERC721ACalls::Symbol(element) => element.fmt(f),
                IERC721ACalls::TokenURI(element) => element.fmt(f),
                IERC721ACalls::TotalSupply(element) => element.fmt(f),
                IERC721ACalls::TransferFrom(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<ApproveCall> for IERC721ACalls {
        fn from(var: ApproveCall) -> Self {
            IERC721ACalls::Approve(var)
        }
    }
    impl ::std::convert::From<BalanceOfCall> for IERC721ACalls {
        fn from(var: BalanceOfCall) -> Self {
            IERC721ACalls::BalanceOf(var)
        }
    }
    impl ::std::convert::From<GetApprovedCall> for IERC721ACalls {
        fn from(var: GetApprovedCall) -> Self {
            IERC721ACalls::GetApproved(var)
        }
    }
    impl ::std::convert::From<IsApprovedForAllCall> for IERC721ACalls {
        fn from(var: IsApprovedForAllCall) -> Self {
            IERC721ACalls::IsApprovedForAll(var)
        }
    }
    impl ::std::convert::From<NameCall> for IERC721ACalls {
        fn from(var: NameCall) -> Self {
            IERC721ACalls::Name(var)
        }
    }
    impl ::std::convert::From<OwnerOfCall> for IERC721ACalls {
        fn from(var: OwnerOfCall) -> Self {
            IERC721ACalls::OwnerOf(var)
        }
    }
    impl ::std::convert::From<SafeTransferFromCall> for IERC721ACalls {
        fn from(var: SafeTransferFromCall) -> Self {
            IERC721ACalls::SafeTransferFrom(var)
        }
    }
    impl ::std::convert::From<SafeTransferFromWithDataCall> for IERC721ACalls {
        fn from(var: SafeTransferFromWithDataCall) -> Self {
            IERC721ACalls::SafeTransferFromWithData(var)
        }
    }
    impl ::std::convert::From<SetApprovalForAllCall> for IERC721ACalls {
        fn from(var: SetApprovalForAllCall) -> Self {
            IERC721ACalls::SetApprovalForAll(var)
        }
    }
    impl ::std::convert::From<SupportsInterfaceCall> for IERC721ACalls {
        fn from(var: SupportsInterfaceCall) -> Self {
            IERC721ACalls::SupportsInterface(var)
        }
    }
    impl ::std::convert::From<SymbolCall> for IERC721ACalls {
        fn from(var: SymbolCall) -> Self {
            IERC721ACalls::Symbol(var)
        }
    }
    impl ::std::convert::From<TokenURICall> for IERC721ACalls {
        fn from(var: TokenURICall) -> Self {
            IERC721ACalls::TokenURI(var)
        }
    }
    impl ::std::convert::From<TotalSupplyCall> for IERC721ACalls {
        fn from(var: TotalSupplyCall) -> Self {
            IERC721ACalls::TotalSupply(var)
        }
    }
    impl ::std::convert::From<TransferFromCall> for IERC721ACalls {
        fn from(var: TransferFromCall) -> Self {
            IERC721ACalls::TransferFrom(var)
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
    pub struct BalanceOfReturn {
        pub balance: ethers::core::types::U256,
    }
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
    pub struct GetApprovedReturn {
        pub operator: ethers::core::types::Address,
    }
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
    pub struct OwnerOfReturn {
        pub owner: ethers::core::types::Address,
    }
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
