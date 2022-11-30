pub use land_nft::*;
#[allow(clippy::too_many_arguments, non_camel_case_types)]
pub mod land_nft {
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
    #[doc = "LandNFT was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    # [rustfmt :: skip] const __ABI : & str = "[{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_devFund\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"_price\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"constructor\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"AddressAlreadyClaimed\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"ApprovalCallerNotOwnerNorApproved\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"ApprovalQueryForNonexistentToken\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"ApproveToCaller\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"BalanceQueryForZeroAddress\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"CannotSetAddressZero\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"ComissionOutOfAllowedRange\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"InsufficientBalance\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"MaxTilesReached\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"MintERC2309QuantityExceedsLimit\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"MintToZeroAddress\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"MintZeroQuantity\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"NonExistentTokenURI\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"NotAuthorised\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"OwnerQueryForNonexistentToken\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"OwnershipNotInitializedForExtraData\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"TransferCallerNotOwnerNorApproved\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"TransferFailed\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"TransferFromIncorrectOwner\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"TransferToNonERC721ReceiverImplementer\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"TransferToZeroAddress\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"URIQueryForNonexistentToken\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"UnauthorizedToMint\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"newAdmin\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"oldAdmin\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"AdminChanged\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"owner\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"approved\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"tokenId\",\"type\":\"uint256\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"Approval\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"owner\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"operator\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"bool\",\"name\":\"approved\",\"type\":\"bool\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"ApprovalForAll\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"newCommission\",\"type\":\"uint256\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"oldCommission\",\"type\":\"uint256\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"CommissionRateChanged\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"fromTokenId\",\"type\":\"uint256\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"toTokenId\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"address\",\"name\":\"from\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"to\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"ConsecutiveTransfer\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"bool\",\"name\":\"paused\",\"type\":\"bool\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"ContractPaused\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"bool\",\"name\":\"paused\",\"type\":\"bool\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"ContractUnpaused\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"newDevFund\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"oldDevFund\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"DevFundChanged\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"newLandBank\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"oldLandBank\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"LandBankChanged\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"buyer\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"struct ILandNFT.Pixel[]\",\"name\":\"region\",\"type\":\"tuple[]\",\"components\":[{\"internalType\":\"struct ILandNFT.Coordonate\",\"name\":\"a\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"uint256\",\"name\":\"lat\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"long\",\"type\":\"uint256\",\"components\":[]}]},{\"internalType\":\"struct ILandNFT.Coordonate\",\"name\":\"b\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"uint256\",\"name\":\"lat\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"long\",\"type\":\"uint256\",\"components\":[]}]},{\"internalType\":\"struct ILandNFT.Coordonate\",\"name\":\"c\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"uint256\",\"name\":\"lat\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"long\",\"type\":\"uint256\",\"components\":[]}]},{\"internalType\":\"struct ILandNFT.Coordonate\",\"name\":\"d\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"uint256\",\"name\":\"lat\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"long\",\"type\":\"uint256\",\"components\":[]}]}],\"indexed\":true}],\"type\":\"event\",\"name\":\"LandSold\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"previousOwner\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"newOwner\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"OwnershipTransferred\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"account\",\"type\":\"address\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"Paused\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"from\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"to\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"tokenId\",\"type\":\"uint256\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"Transfer\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"account\",\"type\":\"address\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"Unpaused\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"MAX_TILE_NUM\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"WETH\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"admin\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"to\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"tokenId\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"approve\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"owner\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"balanceOf\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"baseURI\",\"outputs\":[{\"internalType\":\"string\",\"name\":\"\",\"type\":\"string\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"bytes32[]\",\"name\":\"_merkleProof\",\"type\":\"bytes32[]\",\"components\":[]},{\"internalType\":\"struct ILandNFT.Pixel\",\"name\":\"_region\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"struct ILandNFT.Coordonate\",\"name\":\"a\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"uint256\",\"name\":\"lat\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"long\",\"type\":\"uint256\",\"components\":[]}]},{\"internalType\":\"struct ILandNFT.Coordonate\",\"name\":\"b\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"uint256\",\"name\":\"lat\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"long\",\"type\":\"uint256\",\"components\":[]}]},{\"internalType\":\"struct ILandNFT.Coordonate\",\"name\":\"c\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"uint256\",\"name\":\"lat\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"long\",\"type\":\"uint256\",\"components\":[]}]},{\"internalType\":\"struct ILandNFT.Coordonate\",\"name\":\"d\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"uint256\",\"name\":\"lat\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"long\",\"type\":\"uint256\",\"components\":[]}]}]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"claimSpecialArea\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"commissionRate\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"devFund\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"firstOwners\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_amountIn\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"address[]\",\"name\":\"path\",\"type\":\"address[]\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getAmountOutMin\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"tokenId\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getApproved\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getTokenPrice\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"owner\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"operator\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"isApprovedForAll\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"isOwned\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"landBank\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"merkleRoot\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"struct ILandNFT.Pixel[]\",\"name\":\"region\",\"type\":\"tuple[]\",\"components\":[{\"internalType\":\"struct ILandNFT.Coordonate\",\"name\":\"a\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"uint256\",\"name\":\"lat\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"long\",\"type\":\"uint256\",\"components\":[]}]},{\"internalType\":\"struct ILandNFT.Coordonate\",\"name\":\"b\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"uint256\",\"name\":\"lat\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"long\",\"type\":\"uint256\",\"components\":[]}]},{\"internalType\":\"struct ILandNFT.Coordonate\",\"name\":\"c\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"uint256\",\"name\":\"lat\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"long\",\"type\":\"uint256\",\"components\":[]}]},{\"internalType\":\"struct ILandNFT.Coordonate\",\"name\":\"d\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"uint256\",\"name\":\"lat\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"long\",\"type\":\"uint256\",\"components\":[]}]}]},{\"internalType\":\"uint256\",\"name\":\"rioAmount\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"payable\",\"type\":\"function\",\"name\":\"mint\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"name\",\"outputs\":[{\"internalType\":\"string\",\"name\":\"\",\"type\":\"string\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"nextId\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"owner\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"tokenId\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"ownerOf\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"pause\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"paused\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"price\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"rareClaimedPeriod\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"renounceOwnership\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"from\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"to\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"tokenId\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"safeTransferFrom\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"from\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"to\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"tokenId\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"_data\",\"type\":\"bytes\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"safeTransferFrom\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_newAdmin\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setAdmin\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"operator\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bool\",\"name\":\"approved\",\"type\":\"bool\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setApprovalForAll\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"_baseURI\",\"type\":\"string\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setBaseURI\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_commissionRate\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setCommissionRate\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address payable\",\"name\":\"_devFund\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setDevFund\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address payable\",\"name\":\"_landBank\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setLandBank\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"_root\",\"type\":\"bytes32\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setMerkleRoot\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_price\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setPrice\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"struct ILandNFT.Coordonate\",\"name\":\"a\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"uint256\",\"name\":\"lat\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"long\",\"type\":\"uint256\",\"components\":[]}]},{\"internalType\":\"struct ILandNFT.Coordonate\",\"name\":\"b\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"uint256\",\"name\":\"lat\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"long\",\"type\":\"uint256\",\"components\":[]}]},{\"internalType\":\"struct ILandNFT.Coordonate\",\"name\":\"c\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"uint256\",\"name\":\"lat\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"long\",\"type\":\"uint256\",\"components\":[]}]},{\"internalType\":\"struct ILandNFT.Coordonate\",\"name\":\"d\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"uint256\",\"name\":\"lat\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"long\",\"type\":\"uint256\",\"components\":[]}]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setSpecialArea\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"bytes4\",\"name\":\"interfaceId\",\"type\":\"bytes4\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"supportsInterface\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"symbol\",\"outputs\":[{\"internalType\":\"string\",\"name\":\"\",\"type\":\"string\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"tilesBought\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"tokenId\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"tokenURI\",\"outputs\":[{\"internalType\":\"string\",\"name\":\"\",\"type\":\"string\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"totalSupply\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"totalTileNum\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"from\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"to\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"tokenId\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"transferFrom\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"newOwner\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"transferOwnership\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"unpause\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"payable\",\"type\":\"receive\",\"outputs\":[]}]" ;
    #[doc = r" The parsed JSON-ABI of the contract."]
    pub static LANDNFT_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            ethers::core::utils::__serde_json::from_str(__ABI).expect("invalid abi")
        });
    #[doc = r" Bytecode of the #name contract"]
    pub static LANDNFT_BYTECODE: ethers::contract::Lazy<ethers::core::types::Bytes> =
        ethers::contract::Lazy::new(|| {
            "0x608060405260016009553480156200001657600080fd5b506040516200348038038062003480833981016040819052620000399162000199565b6040518060400160405280600b81526020016a5265616c696f566572736560a81b815250604051806040016040528060048152602001635256525360e01b81525081600290816200008b91906200027a565b5060036200009a82826200027a565b50506000805550620000ac3362000147565b6008805460ff60a01b191690556001600160a01b038216620000e1576040516397b43c7960e01b815260040160405180910390fd5b600b80546001600160a01b0384166001600160a01b031991821617909155600a805490911633178155600f5560108190556040805180820190915260068152655265616c696f60d01b6020820152600d906200013e90826200027a565b50505062000346565b600880546001600160a01b038381166001600160a01b0319831681179093556040519116919082907f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e090600090a35050565b60008060408385031215620001ad57600080fd5b82516001600160a01b0381168114620001c557600080fd5b6020939093015192949293505050565b634e487b7160e01b600052604160045260246000fd5b600181811c908216806200020057607f821691505b6020821081036200022157634e487b7160e01b600052602260045260246000fd5b50919050565b601f8211156200027557600081815260208120601f850160051c81016020861015620002505750805b601f850160051c820191505b8181101562000271578281556001016200025c565b5050505b505050565b81516001600160401b03811115620002965762000296620001d5565b620002ae81620002a78454620001eb565b8462000227565b602080601f831160018114620002e65760008415620002cd5750858301515b600019600386901b1c1916600185901b17855562000271565b600085815260208120601f198616915b828110156200031757888601518255948401946001909101908401620002f6565b5085821015620003365787850151600019600388901b60f8161c191681555b5050505050600190811b01905550565b61312a80620003566000396000f3fe60806040526004361061028c5760003560e01c806370a082311161015a578063b88d4fde116100c1578063efd329731161007a578063efd32973146107b5578063f0640d95146107d5578063f2fde38b146107f5578063f5f7581b14610815578063f851a4401461082e578063fe909f241461084e57600080fd5b8063b88d4fde146106c0578063c427e2b5146106e0578063c457fb37146106f6578063c87b56dd14610716578063e985e9c514610736578063efaff03c1461077f57600080fd5b806395d89b411161011357806395d89b4114610617578063a035b1fe1461062c578063a22cb46514610642578063a4ebcbae14610662578063ad5c464814610678578063ae4db919146106a057600080fd5b806370a082311461056f578063715018a61461058f5780637cb64759146105a45780638456cb59146105c45780638da5cb5b146105d957806391b7f5ed146105f757600080fd5b806342842e0e116101fe5780635ea1d6f8116101b75780635ea1d6f8146104d9578063618b2add146104ef57806361b8ce8c146105045780636352211e1461051a5780636c0360eb1461053a578063704b6c021461054f57600080fd5b806342842e0e1461040a5780634390d2a81461042a578063483344291461044a57806355f804b31461046a578063599f689c1461048a5780635c975abb146104ba57600080fd5b806318160ddd1161025057806318160ddd1461035c57806319fac8fd1461037f5780631e42412a1461039f57806323b872dd146103bf5780632eb4a7ab146103df5780633f4ba83a146103f557600080fd5b8063019ae5991461029857806301ffc9a7146102ad57806306fdde03146102e2578063081812fc14610304578063095ea7b31461033c57600080fd5b3661029357005b600080fd5b6102ab6102a6366004612617565b61086e565b005b3480156102b957600080fd5b506102cd6102c83660046126d4565b6110e8565b60405190151581526020015b60405180910390f35b3480156102ee57600080fd5b506102f761113a565b6040516102d99190612749565b34801561031057600080fd5b5061032461031f36600461275c565b6111cc565b6040516001600160a01b0390911681526020016102d9565b34801561034857600080fd5b506102ab61035736600461278a565b611210565b34801561036857600080fd5b50600154600054035b6040519081526020016102d9565b34801561038b57600080fd5b506102ab61039a36600461275c565b6112b0565b3480156103ab57600080fd5b506102ab6103ba3660046127b6565b611336565b3480156103cb57600080fd5b506102ab6103da36600461283b565b6114cf565b3480156103eb57600080fd5b5061037160135481565b34801561040157600080fd5b506102ab611668565b34801561041657600080fd5b506102ab61042536600461283b565b6116d1565b34801561043657600080fd5b50600b54610324906001600160a01b031681565b34801561045657600080fd5b5061037161046536600461287c565b6116f1565b34801561047657600080fd5b506102ab610485366004612978565b6117a6565b34801561049657600080fd5b506102cd6104a536600461275c565b60166020526000908152604090205460ff1681565b3480156104c657600080fd5b50600854600160a01b900460ff166102cd565b3480156104e557600080fd5b50610371600f5481565b3480156104fb57600080fd5b50601154610371565b34801561051057600080fd5b50610371600e5481565b34801561052657600080fd5b5061032461053536600461275c565b6117be565b34801561054657600080fd5b506102f76117c9565b34801561055b57600080fd5b506102ab61056a3660046129c0565b611857565b34801561057b57600080fd5b5061037161058a3660046129c0565b6118f6565b34801561059b57600080fd5b506102ab611944565b3480156105b057600080fd5b506102ab6105bf36600461275c565b611958565b3480156105d057600080fd5b506102ab611965565b3480156105e557600080fd5b506008546001600160a01b0316610324565b34801561060357600080fd5b506102ab61061236600461275c565b6119ce565b34801561062357600080fd5b506102f7611a02565b34801561063857600080fd5b5061037160105481565b34801561064e57600080fd5b506102ab61065d3660046129eb565b611a11565b34801561066e57600080fd5b5061037160125481565b34801561068457600080fd5b5061032473c02aaa39b223fe8d0a0e5c4f27ead9083c756cc281565b3480156106ac57600080fd5b506102ab6106bb3660046129c0565b611aa6565b3480156106cc57600080fd5b506102ab6106db366004612a24565b611b4f565b3480156106ec57600080fd5b5061037160115481565b34801561070257600080fd5b5061037161071136600461275c565b611b99565b34801561072257600080fd5b506102f761073136600461275c565b611c47565b34801561074257600080fd5b506102cd610751366004612aa3565b6001600160a01b03918216600090815260076020908152604080832093909416825291909152205460ff1690565b34801561078b57600080fd5b5061032461079a36600461275c565b6017602052600090815260409020546001600160a01b031681565b3480156107c157600080fd5b50600c54610324906001600160a01b031681565b3480156107e157600080fd5b506102ab6107f03660046129c0565b611cd7565b34801561080157600080fd5b506102ab6108103660046129c0565b611d80565b34801561082157600080fd5b506103716402540be40081565b34801561083a57600080fd5b50600a54610324906001600160a01b031681565b34801561085a57600080fd5b506102ab610869366004612ad1565b611df6565b8151829060005b8181101561090557600083828151811061089157610891612b2a565b60200260200101516040516020016108a99190612b40565b60408051601f1981840301815291815281516020928301206000818152601990935291205490915060ff16156108f25760405163efbff93960e01b815260040160405180910390fd5b50806108fd81612bb0565b915050610875565b5061090e611e6d565b6402540be4006109216001546000540390565b1061093f5760405163e74fd80b60e01b815260040160405180910390fd5b83518315158061094d575034155b15610e7c5760408051600280825260608201835260009260208301908036833701905050905073f21661d0d1d76d3ecb8e1b9f1c923dbfffae40978160008151811061099b5761099b612b2a565b60200260200101906001600160a01b031690816001600160a01b03168152505073c02aaa39b223fe8d0a0e5c4f27ead9083c756cc2816001815181106109e3576109e3612b2a565b60200260200101906001600160a01b031690816001600160a01b0316815250506000610a16836010546107119190612bc9565b905080861015610a3957604051631e9acf1760e31b815260040160405180910390fd5b60405163095ea7b360e01b81523060048201526024810187905273f21661d0d1d76d3ecb8e1b9f1c923dbfffae40979063095ea7b3906044016020604051808303816000875af1158015610a91573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190610ab59190612be8565b506040516323b872dd60e01b81523360048201523060248201526044810187905260009073f21661d0d1d76d3ecb8e1b9f1c923dbfffae4097906323b872dd906064016020604051808303816000875af1158015610b17573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190610b3b9190612be8565b905080610b5b576040516312171d8360e31b815260040160405180910390fd5b60006064610b6a896014612bc9565b610b749190612c1b565b60405163095ea7b360e01b8152737a250d5630b4cf539739df2c5dacb4c659f2488d60048201526024810182905290915073f21661d0d1d76d3ecb8e1b9f1c923dbfffae40979063095ea7b3906044016020604051808303816000875af1158015610be3573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190610c079190612be8565b50604080516002808252606082018352909160208301908036833701905050935073f21661d0d1d76d3ecb8e1b9f1c923dbfffae409784600081518110610c5057610c50612b2a565b60200260200101906001600160a01b031690816001600160a01b03168152505073c02aaa39b223fe8d0a0e5c4f27ead9083c756cc284600181518110610c9857610c98612b2a565b60200260200101906001600160a01b031690816001600160a01b0316815250506000610cc482866116f1565b600b546040516318cbafe560e01b8152919250737a250d5630b4cf539739df2c5dacb4c659f2488d916318cbafe591610d1191869186918b916001600160a01b0316904290600401612c73565b600060405180830381600087803b158015610d2b57600080fd5b505af1158015610d3f573d6000803e3d6000fd5b5050600c5473f21661d0d1d76d3ecb8e1b9f1c923dbfffae4097925063a9059cbb91506001600160a01b03166064610d788d6050612bc9565b610d829190612c1b565b6040516001600160e01b031960e085901b1681526001600160a01b03909216600483015260248201526044016020604051808303816000875af1158015610dcd573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190610df19190612be8565b925082610e11576040516312171d8360e31b815260040160405180910390fd5b610e1b3387611eba565b8560116000828254610e2d9190612caf565b9091555050604051610e40908b90612cc7565b6040519081900381209033907ffedaf99886e624862075ed4c59d51e3ec75e512a2ddd085a09a1234f2a486bc790600090a350505050506110e1565b6000341180610e89575083155b156110e1578451601054610e9d9190612bc9565b341015610ebd57604051631e9acf1760e31b815260040160405180910390fd5b600b546001600160a01b03166108fc6064610ed9346014612bc9565b610ee39190612c1b565b6040518115909202916000818181858888f19350505050158015610f0b573d6000803e3d6000fd5b5060006064610f1b346050612bc9565b610f259190612c1b565b604080516002808252606082018352929350600092909160208301908036833701905050905073c02aaa39b223fe8d0a0e5c4f27ead9083c756cc281600081518110610f7357610f73612b2a565b60200260200101906001600160a01b031690816001600160a01b03168152505073f21661d0d1d76d3ecb8e1b9f1c923dbfffae409781600181518110610fbb57610fbb612b2a565b60200260200101906001600160a01b031690816001600160a01b0316815250506000610fe783836116f1565b600c54604051637ff36ab560e01b8152919250737a250d5630b4cf539739df2c5dacb4c659f2488d91637ff36ab591869161103491869188916001600160a01b0316904290600401612d5f565b60006040518083038185885af1158015611052573d6000803e3d6000fd5b50505050506040513d6000823e601f3d908101601f1916820160405261107b9190810190612d94565b506110863385611eba565b83601160008282546110989190612caf565b90915550506040516110ab908990612cc7565b6040519081900381209033907ffedaf99886e624862075ed4c59d51e3ec75e512a2ddd085a09a1234f2a486bc790600090a35050505b5050505050565b60006301ffc9a760e01b6001600160e01b03198316148061111957506380ac58cd60e01b6001600160e01b03198316145b806111345750635b5e139f60e01b6001600160e01b03198316145b92915050565b60606002805461114990612e24565b80601f016020809104026020016040519081016040528092919081815260200182805461117590612e24565b80156111c25780601f10611197576101008083540402835291602001916111c2565b820191906000526020600020905b8154815290600101906020018083116111a557829003601f168201915b5050505050905090565b60006111d782611ed4565b6111f4576040516333d1c03960e21b815260040160405180910390fd5b506000908152600660205260409020546001600160a01b031690565b600061121b826117be565b9050336001600160a01b03821614611254576112378133610751565b611254576040516367d9dca160e11b815260040160405180910390fd5b60008281526006602052604080822080546001600160a01b0319166001600160a01b0387811691821790925591518593918516917f8c5be1e5ebec7d5bd14f71427d1e84f3dd0314c0f7b2291e5b200ac8c7c3b92591a4505050565b600f54600a546001600160a01b031633146112de57604051631648fd0160e01b815260040160405180910390fd5b601482111561130057604051637d1dd3df60e11b815260040160405180910390fd5b600f829055604051819083907f74b81a9e0217358c4b0755d3032738dc303e980dde2980905160b1d8e7b68ba690600090a35050565b336000908152601b602052604090205460ff16151560010361136b57604051635986e84960e01b815260040160405180910390fd5b6040516bffffffffffffffffffffffff193360601b1660208201526000906034016040516020818303038152906040528051906020012090506000826040516020016113b79190612b40565b60408051601f1981840301815291815281516020928301206000818152601a90935291205490915060ff161561142b5760405162461bcd60e51b8152602060048201526014602482015273185c995848185b1c9958591e4818db185a5b595960621b60448201526064015b60405180910390fd5b61146c858580806020026020016040519081016040528093929190818152602001838360200280828437600092019190915250506013549150859050611efb565b6114a85760405162461bcd60e51b815260206004820152600d60248201526c34b73b30b634b210383937b7b360991b6044820152606401611422565b336000818152601b60205260409020805460ff191660019081179091556110e19190611eba565b60006114da82611f11565b9050836001600160a01b0316816001600160a01b03161461150d5760405162a1148160e81b815260040160405180910390fd5b60008281526006602052604090208054338082146001600160a01b0388169091141761155a5761153d8633610751565b61155a57604051632ce44b5f60e11b815260040160405180910390fd5b6001600160a01b03851661158157604051633a954ecd60e21b815260040160405180910390fd5b801561158c57600082555b6001600160a01b038681166000908152600560205260408082208054600019019055918716808252919020805460010190554260a01b17600160e11b17600085815260046020526040812091909155600160e11b8416900361161e5760018401600081815260046020526040812054900361161c57600054811461161c5760008181526004602052604090208490555b505b83856001600160a01b0316876001600160a01b03167fddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef60405160405180910390a45b505050505050565b611670611f7f565b600a546001600160a01b0316331461169b57604051631648fd0160e01b815260040160405180910390fd5b6116a3611fcf565b6040516001907fc6cd34d367248623c114617f3cf4e7d54b15f11b158367408ee3b4c0ff1a5e2e90600090a2565b6116ec83838360405180602001604052806000815250611b4f565b505050565b60405163d06ca61f60e01b81526000908190737a250d5630b4cf539739df2c5dacb4c659f2488d9063d06ca61f9061172f9087908790600401612e5e565b600060405180830381865afa15801561174c573d6000803e3d6000fd5b505050506040513d6000823e601f3d908101601f191682016040526117749190810190612d94565b905080600184516117859190612e77565b8151811061179557611795612b2a565b602002602001015191505092915050565b6117ae612024565b600d6117ba8282612ed4565b5050565b600061113482611f11565b600d80546117d690612e24565b80601f016020809104026020016040519081016040528092919081815260200182805461180290612e24565b801561184f5780601f106118245761010080835404028352916020019161184f565b820191906000526020600020905b81548152906001019060200180831161183257829003601f168201915b505050505081565b600a546001600160a01b0316331461188257604051631648fd0160e01b815260040160405180910390fd5b6001600160a01b0381166118a9576040516397b43c7960e01b815260040160405180910390fd5b600a80546001600160a01b0319166001600160a01b0383169081179091556040513391907f7e644d79422f17c01e4894b5f4f588d331ebfa28653d42ae832dc59e38c9798f90600090a350565b60006001600160a01b03821661191f576040516323d3ad8160e21b815260040160405180910390fd5b506001600160a01b03166000908152600560205260409020546001600160401b031690565b61194c612024565b611956600061207e565b565b611960612024565b601355565b61196d611e6d565b600a546001600160a01b0316331461199857604051631648fd0160e01b815260040160405180910390fd5b6119a06120d0565b6040516001907f752d7e161ff5146f80e3820893176eb40532811e5e20400dfdde57455213706a90600090a2565b600a546001600160a01b031633146119f957604051631648fd0160e01b815260040160405180910390fd5b60108190555b50565b60606003805461114990612e24565b336001600160a01b03831603611a3a5760405163b06307db60e01b815260040160405180910390fd5b3360008181526007602090815260408083206001600160a01b03871680855290835292819020805460ff191686151590811790915590519081529192917f17307eab39ab6107e8899845ad3d59bd9653f200f220920489ca2b5937696c31910160405180910390a35050565b600b54600a546001600160a01b0391821691163314611ad857604051631648fd0160e01b815260040160405180910390fd5b6001600160a01b038216611aff576040516397b43c7960e01b815260040160405180910390fd5b600b80546001600160a01b0319166001600160a01b03848116918217909255604051918316917ff87e12ba363db684b1b69a530d850a8a3f416932cd031e008ef71e42a1d8845090600090a35050565b611b5a8484846114cf565b6001600160a01b0383163b15611b9357611b7684848484612113565b611b93576040516368d2bf6b60e11b815260040160405180910390fd5b50505050565b600080730b85b3000bef3e26e01428d1b525a532ea7513b89050600080826001600160a01b0316630902f1ac6040518163ffffffff1660e01b8152600401606060405180830381865afa158015611bf4573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190611c189190612faf565b506001600160701b03918216935016905081611c348287612bc9565b611c3e9190612c1b565b95945050505050565b60606000611c54836117be565b6001600160a01b031603611c7b5760405163d872946b60e01b815260040160405180910390fd5b6000600d8054611c8a90612e24565b905011611ca65760405180602001604052806000815250611134565b600d611cb1836121ff565b604051602001611cc2929190612fff565b60405160208183030381529060405292915050565b600c54600a546001600160a01b0391821691163314611d0957604051631648fd0160e01b815260040160405180910390fd5b6001600160a01b038216611d30576040516397b43c7960e01b815260040160405180910390fd5b600c80546001600160a01b0319166001600160a01b03848116918217909255604051918316917f04c90a5bd107b5b753ce9758599ca56cffaadede0b5f6c4b3a375a5effe208d490600090a35050565b611d88612024565b6001600160a01b038116611ded5760405162461bcd60e51b815260206004820152602660248201527f4f776e61626c653a206e6577206f776e657220697320746865207a65726f206160448201526564647265737360d01b6064820152608401611422565b6119ff8161207e565b611dfe612024565b60006040518060800160405280868152602001858152602001848152602001838152509050600081604051602001611e369190612b40565b60408051601f198184030181529181528151602092830120600090815260199092529020805460ff19166001179055505050505050565b600854600160a01b900460ff16156119565760405162461bcd60e51b815260206004820152601060248201526f14185d5cd8589b194e881c185d5cd95960821b6044820152606401611422565b6117ba8282604051806020016040528060008152506122ff565b6000805482108015611134575050600090815260046020526040902054600160e01b161590565b600082611f088584612365565b14949350505050565b600081600054811015611f665760008181526004602052604081205490600160e01b82169003611f64575b80600003611f5d575060001901600081815260046020526040902054611f3c565b9392505050565b505b604051636f96cda160e11b815260040160405180910390fd5b600854600160a01b900460ff166119565760405162461bcd60e51b815260206004820152601460248201527314185d5cd8589b194e881b9bdd081c185d5cd95960621b6044820152606401611422565b611fd7611f7f565b6008805460ff60a01b191690557f5db9ee0a495bf2e6ff9c91a7834c1ba4fdd244a5e8aa4e537bd38aeae4b073aa335b6040516001600160a01b03909116815260200160405180910390a1565b6008546001600160a01b031633146119565760405162461bcd60e51b815260206004820181905260248201527f4f776e61626c653a2063616c6c6572206973206e6f7420746865206f776e65726044820152606401611422565b600880546001600160a01b038381166001600160a01b0319831681179093556040519116919082907f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e090600090a35050565b6120d8611e6d565b6008805460ff60a01b1916600160a01b1790557f62e78cea01bee320cd4e420270b5ea74000d11b0c9f74754ebdbfc544b05a2586120073390565b604051630a85bd0160e11b81526000906001600160a01b0385169063150b7a0290612148903390899088908890600401613086565b6020604051808303816000875af1925050508015612183575060408051601f3d908101601f19168201909252612180918101906130c3565b60015b6121e1573d8080156121b1576040519150601f19603f3d011682016040523d82523d6000602084013e6121b6565b606091505b5080516000036121d9576040516368d2bf6b60e11b815260040160405180910390fd5b805181602001fd5b6001600160e01b031916630a85bd0160e11b1490505b949350505050565b6060816000036122265750506040805180820190915260018152600360fc1b602082015290565b8160005b8115612250578061223a81612bb0565b91506122499050600a83612c1b565b915061222a565b6000816001600160401b0381111561226a5761226a6124dc565b6040519080825280601f01601f191660200182016040528015612294576020820181803683370190505b5090505b84156121f7576122a9600183612e77565b91506122b6600a866130e0565b6122c1906030612caf565b60f81b8183815181106122d6576122d6612b2a565b60200101906001600160f81b031916908160001a9053506122f8600a86612c1b565b9450612298565b61230983836123b2565b6001600160a01b0383163b156116ec576000548281035b6123336000868380600101945086612113565b612350576040516368d2bf6b60e11b815260040160405180910390fd5b8181106123205781600054146110e157600080fd5b600081815b84518110156123aa576123968286838151811061238957612389612b2a565b60200260200101516124b0565b9150806123a281612bb0565b91505061236a565b509392505050565b60008054908290036123d75760405163b562e8dd60e01b815260040160405180910390fd5b6001600160a01b03831660008181526005602090815260408083208054680100000000000000018802019055848352600490915281206001851460e11b4260a01b178317905582840190839083907fddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef8180a4600183015b81811461248657808360007fddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef600080a460010161244e565b50816000036124a757604051622e076360e81b815260040160405180910390fd5b60005550505050565b60008183106124cc576000828152602084905260409020611f5d565b5060009182526020526040902090565b634e487b7160e01b600052604160045260246000fd5b604051601f8201601f191681016001600160401b038111828210171561251a5761251a6124dc565b604052919050565b60006001600160401b0382111561253b5761253b6124dc565b5060051b60200190565b60006040828403121561255757600080fd5b604051604081018181106001600160401b0382111715612579576125796124dc565b604052823581526020928301359281019290925250919050565b600061010082840312156125a657600080fd5b604051608081018181106001600160401b03821117156125c8576125c86124dc565b6040529050806125d88484612545565b81526125e78460408501612545565b60208201526125f98460808501612545565b604082015261260b8460c08501612545565b60608201525092915050565b6000806040838503121561262a57600080fd5b82356001600160401b0381111561264057600080fd5b8301601f8101851361265157600080fd5b8035602061266661266183612522565b6124f2565b82815260089290921b8301810191818101908884111561268557600080fd5b938201935b838510156126af5761269c8986612593565b825282820191506101008501945061268a565b98969091013596505050505050565b6001600160e01b0319811681146119ff57600080fd5b6000602082840312156126e657600080fd5b8135611f5d816126be565b60005b8381101561270c5781810151838201526020016126f4565b83811115611b935750506000910152565b600081518084526127358160208601602086016126f1565b601f01601f19169290920160200192915050565b602081526000611f5d602083018461271d565b60006020828403121561276e57600080fd5b5035919050565b6001600160a01b03811681146119ff57600080fd5b6000806040838503121561279d57600080fd5b82356127a881612775565b946020939093013593505050565b600080600061012084860312156127cc57600080fd5b83356001600160401b03808211156127e357600080fd5b818601915086601f8301126127f757600080fd5b81358181111561280657600080fd5b8760208260051b850101111561281b57600080fd5b602092830195509350612832918791508601612593565b90509250925092565b60008060006060848603121561285057600080fd5b833561285b81612775565b9250602084013561286b81612775565b929592945050506040919091013590565b6000806040838503121561288f57600080fd5b823591506020808401356001600160401b038111156128ad57600080fd5b8401601f810186136128be57600080fd5b80356128cc61266182612522565b81815260059190911b820183019083810190888311156128eb57600080fd5b928401925b8284101561291257833561290381612775565b825292840192908401906128f0565b80955050505050509250929050565b60006001600160401b0383111561293a5761293a6124dc565b61294d601f8401601f19166020016124f2565b905082815283838301111561296157600080fd5b828260208301376000602084830101529392505050565b60006020828403121561298a57600080fd5b81356001600160401b038111156129a057600080fd5b8201601f810184136129b157600080fd5b6121f784823560208401612921565b6000602082840312156129d257600080fd5b8135611f5d81612775565b80151581146119ff57600080fd5b600080604083850312156129fe57600080fd5b8235612a0981612775565b91506020830135612a19816129dd565b809150509250929050565b60008060008060808587031215612a3a57600080fd5b8435612a4581612775565b93506020850135612a5581612775565b92506040850135915060608501356001600160401b03811115612a7757600080fd5b8501601f81018713612a8857600080fd5b612a9787823560208401612921565b91505092959194509250565b60008060408385031215612ab657600080fd5b8235612ac181612775565b91506020830135612a1981612775565b6000806000806101008587031215612ae857600080fd5b612af28686612545565b9350612b018660408701612545565b9250612b108660808701612545565b9150612b1f8660c08701612545565b905092959194509250565b634e487b7160e01b600052603260045260246000fd5b8151805182526020908101518183015280830151805160408085019190915290820151606080850191909152908401518051608085015282015160a084015290920151805160c08301529091015160e08201526101000190565b634e487b7160e01b600052601160045260246000fd5b600060018201612bc257612bc2612b9a565b5060010190565b6000816000190483118215151615612be357612be3612b9a565b500290565b600060208284031215612bfa57600080fd5b8151611f5d816129dd565b634e487b7160e01b600052601260045260246000fd5b600082612c2a57612c2a612c05565b500490565b600081518084526020808501945080840160005b83811015612c685781516001600160a01b031687529582019590820190600101612c43565b509495945050505050565b85815284602082015260a060408201526000612c9260a0830186612c2f565b6001600160a01b0394909416606083015250608001529392505050565b60008219821115612cc257612cc2612b9a565b500190565b815160009082906020808601845b83811015612d5357815180518051875260209081015181880152848201518051604089015201516060870152612d3d612d24608088016040840151805182526020908101519082015260400190565b6060830151805182526020908101519082015260400190565b5050610100949094019390820190600101612cd5565b50929695505050505050565b848152608060208201526000612d786080830186612c2f565b6001600160a01b03949094166040830152506060015292915050565b60006020808385031215612da757600080fd5b82516001600160401b03811115612dbd57600080fd5b8301601f81018513612dce57600080fd5b8051612ddc61266182612522565b81815260059190911b82018301908381019087831115612dfb57600080fd5b928401925b82841015612e1957835182529284019290840190612e00565b979650505050505050565b600181811c90821680612e3857607f821691505b602082108103612e5857634e487b7160e01b600052602260045260246000fd5b50919050565b8281526040602082015260006121f76040830184612c2f565b600082821015612e8957612e89612b9a565b500390565b601f8211156116ec57600081815260208120601f850160051c81016020861015612eb55750805b601f850160051c820191505b8181101561166057828155600101612ec1565b81516001600160401b03811115612eed57612eed6124dc565b612f0181612efb8454612e24565b84612e8e565b602080601f831160018114612f365760008415612f1e5750858301515b600019600386901b1c1916600185901b178555611660565b600085815260208120601f198616915b82811015612f6557888601518255948401946001909101908401612f46565b5085821015612f835787850151600019600388901b60f8161c191681555b5050505050600190811b01905550565b80516001600160701b0381168114612faa57600080fd5b919050565b600080600060608486031215612fc457600080fd5b612fcd84612f93565b9250612fdb60208501612f93565b9150604084015163ffffffff81168114612ff457600080fd5b809150509250925092565b600080845461300d81612e24565b60018281168015613025576001811461303a57613069565b60ff1984168752821515830287019450613069565b8860005260208060002060005b858110156130605781548a820152908401908201613047565b50505082870194505b50505050835161307d8183602088016126f1565b01949350505050565b6001600160a01b03858116825284166020820152604081018390526080606082018190526000906130b99083018461271d565b9695505050505050565b6000602082840312156130d557600080fd5b8151611f5d816126be565b6000826130ef576130ef612c05565b50069056fea264697066735822122068e36d1c925c950f4ed99837534304ce6a4ffeed5e88d2268dfb9fc72dc9e21664736f6c634300080f0033" . parse () . expect ("invalid bytecode")
        });
    pub struct LandNFT<M>(ethers::contract::Contract<M>);
    impl<M> Clone for LandNFT<M> {
        fn clone(&self) -> Self {
            LandNFT(self.0.clone())
        }
    }
    impl<M> std::ops::Deref for LandNFT<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> std::fmt::Debug for LandNFT<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(LandNFT))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ethers::providers::Middleware> LandNFT<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(address.into(), LANDNFT_ABI.clone(), client).into()
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
                LANDNFT_ABI.clone(),
                LANDNFT_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        #[doc = "Calls the contract's `MAX_TILE_NUM` (0xf5f7581b) function"]
        pub fn max_tile_num(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([245, 247, 88, 27], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `WETH` (0xad5c4648) function"]
        pub fn weth(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([173, 92, 70, 72], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `admin` (0xf851a440) function"]
        pub fn admin(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([248, 81, 164, 64], ())
                .expect("method not found (this should never happen)")
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
        #[doc = "Calls the contract's `baseURI` (0x6c0360eb) function"]
        pub fn base_uri(&self) -> ethers::contract::builders::ContractCall<M, String> {
            self.0
                .method_hash([108, 3, 96, 235], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `claimSpecialArea` (0x1e42412a) function"]
        pub fn claim_special_area(
            &self,
            merkle_proof: ::std::vec::Vec<[u8; 32]>,
            region: Pixel,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([30, 66, 65, 42], (merkle_proof, region))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `commissionRate` (0x5ea1d6f8) function"]
        pub fn commission_rate(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([94, 161, 214, 248], ())
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
        #[doc = "Calls the contract's `firstOwners` (0xefaff03c) function"]
        pub fn first_owners(
            &self,
            p0: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([239, 175, 240, 60], p0)
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
        #[doc = "Calls the contract's `getApproved` (0x081812fc) function"]
        pub fn get_approved(
            &self,
            token_id: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([8, 24, 18, 252], token_id)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getTokenPrice` (0xc457fb37) function"]
        pub fn get_token_price(
            &self,
            amount: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([196, 87, 251, 55], amount)
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
        #[doc = "Calls the contract's `isOwned` (0x599f689c) function"]
        pub fn is_owned(
            &self,
            p0: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([89, 159, 104, 156], p0)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `landBank` (0xefd32973) function"]
        pub fn land_bank(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([239, 211, 41, 115], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `merkleRoot` (0x2eb4a7ab) function"]
        pub fn merkle_root(&self) -> ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([46, 180, 167, 171], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `mint` (0x019ae599) function"]
        pub fn mint(
            &self,
            region: ::std::vec::Vec<Pixel>,
            rio_amount: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([1, 154, 229, 153], (region, rio_amount))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `name` (0x06fdde03) function"]
        pub fn name(&self) -> ethers::contract::builders::ContractCall<M, String> {
            self.0
                .method_hash([6, 253, 222, 3], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `nextId` (0x61b8ce8c) function"]
        pub fn next_id(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([97, 184, 206, 140], ())
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
        #[doc = "Calls the contract's `ownerOf` (0x6352211e) function"]
        pub fn owner_of(
            &self,
            token_id: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([99, 82, 33, 30], token_id)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `pause` (0x8456cb59) function"]
        pub fn pause(&self) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([132, 86, 203, 89], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `paused` (0x5c975abb) function"]
        pub fn paused(&self) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([92, 151, 90, 187], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `price` (0xa035b1fe) function"]
        pub fn price(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([160, 53, 177, 254], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `rareClaimedPeriod` (0xa4ebcbae) function"]
        pub fn rare_claimed_period(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([164, 235, 203, 174], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `renounceOwnership` (0x715018a6) function"]
        pub fn renounce_ownership(&self) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([113, 80, 24, 166], ())
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
        #[doc = "Calls the contract's `setAdmin` (0x704b6c02) function"]
        pub fn set_admin(
            &self,
            new_admin: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([112, 75, 108, 2], new_admin)
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
        #[doc = "Calls the contract's `setBaseURI` (0x55f804b3) function"]
        pub fn set_base_uri(
            &self,
            base_uri: String,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([85, 248, 4, 179], base_uri)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setCommissionRate` (0x19fac8fd) function"]
        pub fn set_commission_rate(
            &self,
            commission_rate: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([25, 250, 200, 253], commission_rate)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setDevFund` (0xae4db919) function"]
        pub fn set_dev_fund(
            &self,
            dev_fund: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([174, 77, 185, 25], dev_fund)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setLandBank` (0xf0640d95) function"]
        pub fn set_land_bank(
            &self,
            land_bank: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([240, 100, 13, 149], land_bank)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setMerkleRoot` (0x7cb64759) function"]
        pub fn set_merkle_root(
            &self,
            root: [u8; 32],
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([124, 182, 71, 89], root)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setPrice` (0x91b7f5ed) function"]
        pub fn set_price(
            &self,
            price: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([145, 183, 245, 237], price)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setSpecialArea` (0xfe909f24) function"]
        pub fn set_special_area(
            &self,
            a: Coordonate,
            b: Coordonate,
            c: Coordonate,
            d: Coordonate,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([254, 144, 159, 36], (a, b, c, d))
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
        #[doc = "Calls the contract's `tilesBought` (0xc427e2b5) function"]
        pub fn tiles_bought(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([196, 39, 226, 181], ())
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
        #[doc = "Calls the contract's `totalTileNum` (0x618b2add) function"]
        pub fn total_tile_num(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([97, 139, 42, 221], ())
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
        #[doc = "Calls the contract's `transferOwnership` (0xf2fde38b) function"]
        pub fn transfer_ownership(
            &self,
            new_owner: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([242, 253, 227, 139], new_owner)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `unpause` (0x3f4ba83a) function"]
        pub fn unpause(&self) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([63, 75, 168, 58], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Gets the contract's `AdminChanged` event"]
        pub fn admin_changed_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, AdminChangedFilter> {
            self.0.event()
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
        #[doc = "Gets the contract's `CommissionRateChanged` event"]
        pub fn commission_rate_changed_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, CommissionRateChangedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `ConsecutiveTransfer` event"]
        pub fn consecutive_transfer_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, ConsecutiveTransferFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `ContractPaused` event"]
        pub fn contract_paused_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, ContractPausedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `ContractUnpaused` event"]
        pub fn contract_unpaused_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, ContractUnpausedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `DevFundChanged` event"]
        pub fn dev_fund_changed_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, DevFundChangedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `LandBankChanged` event"]
        pub fn land_bank_changed_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, LandBankChangedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `LandSold` event"]
        pub fn land_sold_filter(&self) -> ethers::contract::builders::Event<M, LandSoldFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `OwnershipTransferred` event"]
        pub fn ownership_transferred_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, OwnershipTransferredFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `Paused` event"]
        pub fn paused_filter(&self) -> ethers::contract::builders::Event<M, PausedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `Transfer` event"]
        pub fn transfer_filter(&self) -> ethers::contract::builders::Event<M, TransferFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `Unpaused` event"]
        pub fn unpaused_filter(&self) -> ethers::contract::builders::Event<M, UnpausedFilter> {
            self.0.event()
        }
        #[doc = r" Returns an [`Event`](#ethers_contract::builders::Event) builder for all events of this contract"]
        pub fn events(&self) -> ethers::contract::builders::Event<M, LandNFTEvents> {
            self.0.event_with_filter(Default::default())
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>> for LandNFT<M> {
        fn from(contract: ethers::contract::Contract<M>) -> Self {
            Self(contract)
        }
    }
    #[doc = "Custom Error type `AddressAlreadyClaimed` with signature `AddressAlreadyClaimed()` and selector `[89, 134, 232, 73]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthError,
        ethers :: contract :: EthDisplay,
    )]
    #[etherror(name = "AddressAlreadyClaimed", abi = "AddressAlreadyClaimed()")]
    pub struct AddressAlreadyClaimed;
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
    #[doc = "Custom Error type `CannotSetAddressZero` with signature `CannotSetAddressZero()` and selector `[151, 180, 60, 121]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthError,
        ethers :: contract :: EthDisplay,
    )]
    #[etherror(name = "CannotSetAddressZero", abi = "CannotSetAddressZero()")]
    pub struct CannotSetAddressZero;
    #[doc = "Custom Error type `ComissionOutOfAllowedRange` with signature `ComissionOutOfAllowedRange()` and selector `[250, 59, 167, 190]`"]
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
        name = "ComissionOutOfAllowedRange",
        abi = "ComissionOutOfAllowedRange()"
    )]
    pub struct ComissionOutOfAllowedRange;
    #[doc = "Custom Error type `InsufficientBalance` with signature `InsufficientBalance()` and selector `[244, 214, 120, 184]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthError,
        ethers :: contract :: EthDisplay,
    )]
    #[etherror(name = "InsufficientBalance", abi = "InsufficientBalance()")]
    pub struct InsufficientBalance;
    #[doc = "Custom Error type `MaxTilesReached` with signature `MaxTilesReached()` and selector `[231, 79, 216, 11]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthError,
        ethers :: contract :: EthDisplay,
    )]
    #[etherror(name = "MaxTilesReached", abi = "MaxTilesReached()")]
    pub struct MaxTilesReached;
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
    #[doc = "Custom Error type `NonExistentTokenURI` with signature `NonExistentTokenURI()` and selector `[216, 114, 148, 107]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthError,
        ethers :: contract :: EthDisplay,
    )]
    #[etherror(name = "NonExistentTokenURI", abi = "NonExistentTokenURI()")]
    pub struct NonExistentTokenURI;
    #[doc = "Custom Error type `NotAuthorised` with signature `NotAuthorised()` and selector `[22, 72, 253, 1]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthError,
        ethers :: contract :: EthDisplay,
    )]
    #[etherror(name = "NotAuthorised", abi = "NotAuthorised()")]
    pub struct NotAuthorised;
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
    #[doc = "Custom Error type `TransferFailed` with signature `TransferFailed()` and selector `[144, 184, 236, 24]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthError,
        ethers :: contract :: EthDisplay,
    )]
    #[etherror(name = "TransferFailed", abi = "TransferFailed()")]
    pub struct TransferFailed;
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
    #[doc = "Custom Error type `UnauthorizedToMint` with signature `UnauthorizedToMint()` and selector `[239, 191, 249, 57]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthError,
        ethers :: contract :: EthDisplay,
    )]
    #[etherror(name = "UnauthorizedToMint", abi = "UnauthorizedToMint()")]
    pub struct UnauthorizedToMint;
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum LandNFTErrors {
        AddressAlreadyClaimed(AddressAlreadyClaimed),
        ApprovalCallerNotOwnerNorApproved(ApprovalCallerNotOwnerNorApproved),
        ApprovalQueryForNonexistentToken(ApprovalQueryForNonexistentToken),
        ApproveToCaller(ApproveToCaller),
        BalanceQueryForZeroAddress(BalanceQueryForZeroAddress),
        CannotSetAddressZero(CannotSetAddressZero),
        ComissionOutOfAllowedRange(ComissionOutOfAllowedRange),
        InsufficientBalance(InsufficientBalance),
        MaxTilesReached(MaxTilesReached),
        MintERC2309QuantityExceedsLimit(MintERC2309QuantityExceedsLimit),
        MintToZeroAddress(MintToZeroAddress),
        MintZeroQuantity(MintZeroQuantity),
        NonExistentTokenURI(NonExistentTokenURI),
        NotAuthorised(NotAuthorised),
        OwnerQueryForNonexistentToken(OwnerQueryForNonexistentToken),
        OwnershipNotInitializedForExtraData(OwnershipNotInitializedForExtraData),
        TransferCallerNotOwnerNorApproved(TransferCallerNotOwnerNorApproved),
        TransferFailed(TransferFailed),
        TransferFromIncorrectOwner(TransferFromIncorrectOwner),
        TransferToNonERC721ReceiverImplementer(TransferToNonERC721ReceiverImplementer),
        TransferToZeroAddress(TransferToZeroAddress),
        URIQueryForNonexistentToken(URIQueryForNonexistentToken),
        UnauthorizedToMint(UnauthorizedToMint),
    }
    impl ethers::core::abi::AbiDecode for LandNFTErrors {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::std::result::Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <AddressAlreadyClaimed as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(LandNFTErrors::AddressAlreadyClaimed(decoded));
            }
            if let Ok(decoded) =
                <ApprovalCallerNotOwnerNorApproved as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(LandNFTErrors::ApprovalCallerNotOwnerNorApproved(decoded));
            }
            if let Ok(decoded) =
                <ApprovalQueryForNonexistentToken as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(LandNFTErrors::ApprovalQueryForNonexistentToken(decoded));
            }
            if let Ok(decoded) =
                <ApproveToCaller as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(LandNFTErrors::ApproveToCaller(decoded));
            }
            if let Ok(decoded) =
                <BalanceQueryForZeroAddress as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(LandNFTErrors::BalanceQueryForZeroAddress(decoded));
            }
            if let Ok(decoded) =
                <CannotSetAddressZero as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(LandNFTErrors::CannotSetAddressZero(decoded));
            }
            if let Ok(decoded) =
                <ComissionOutOfAllowedRange as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(LandNFTErrors::ComissionOutOfAllowedRange(decoded));
            }
            if let Ok(decoded) =
                <InsufficientBalance as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(LandNFTErrors::InsufficientBalance(decoded));
            }
            if let Ok(decoded) =
                <MaxTilesReached as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(LandNFTErrors::MaxTilesReached(decoded));
            }
            if let Ok(decoded) =
                <MintERC2309QuantityExceedsLimit as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(LandNFTErrors::MintERC2309QuantityExceedsLimit(decoded));
            }
            if let Ok(decoded) =
                <MintToZeroAddress as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(LandNFTErrors::MintToZeroAddress(decoded));
            }
            if let Ok(decoded) =
                <MintZeroQuantity as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(LandNFTErrors::MintZeroQuantity(decoded));
            }
            if let Ok(decoded) =
                <NonExistentTokenURI as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(LandNFTErrors::NonExistentTokenURI(decoded));
            }
            if let Ok(decoded) =
                <NotAuthorised as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(LandNFTErrors::NotAuthorised(decoded));
            }
            if let Ok(decoded) =
                <OwnerQueryForNonexistentToken as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(LandNFTErrors::OwnerQueryForNonexistentToken(decoded));
            }
            if let Ok(decoded) =
                <OwnershipNotInitializedForExtraData as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(LandNFTErrors::OwnershipNotInitializedForExtraData(decoded));
            }
            if let Ok(decoded) =
                <TransferCallerNotOwnerNorApproved as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(LandNFTErrors::TransferCallerNotOwnerNorApproved(decoded));
            }
            if let Ok(decoded) =
                <TransferFailed as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(LandNFTErrors::TransferFailed(decoded));
            }
            if let Ok(decoded) =
                <TransferFromIncorrectOwner as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(LandNFTErrors::TransferFromIncorrectOwner(decoded));
            }
            if let Ok(decoded) =
                <TransferToNonERC721ReceiverImplementer as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(LandNFTErrors::TransferToNonERC721ReceiverImplementer(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <TransferToZeroAddress as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(LandNFTErrors::TransferToZeroAddress(decoded));
            }
            if let Ok(decoded) =
                <URIQueryForNonexistentToken as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(LandNFTErrors::URIQueryForNonexistentToken(decoded));
            }
            if let Ok(decoded) =
                <UnauthorizedToMint as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(LandNFTErrors::UnauthorizedToMint(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for LandNFTErrors {
        fn encode(self) -> Vec<u8> {
            match self {
                LandNFTErrors::AddressAlreadyClaimed(element) => element.encode(),
                LandNFTErrors::ApprovalCallerNotOwnerNorApproved(element) => element.encode(),
                LandNFTErrors::ApprovalQueryForNonexistentToken(element) => element.encode(),
                LandNFTErrors::ApproveToCaller(element) => element.encode(),
                LandNFTErrors::BalanceQueryForZeroAddress(element) => element.encode(),
                LandNFTErrors::CannotSetAddressZero(element) => element.encode(),
                LandNFTErrors::ComissionOutOfAllowedRange(element) => element.encode(),
                LandNFTErrors::InsufficientBalance(element) => element.encode(),
                LandNFTErrors::MaxTilesReached(element) => element.encode(),
                LandNFTErrors::MintERC2309QuantityExceedsLimit(element) => element.encode(),
                LandNFTErrors::MintToZeroAddress(element) => element.encode(),
                LandNFTErrors::MintZeroQuantity(element) => element.encode(),
                LandNFTErrors::NonExistentTokenURI(element) => element.encode(),
                LandNFTErrors::NotAuthorised(element) => element.encode(),
                LandNFTErrors::OwnerQueryForNonexistentToken(element) => element.encode(),
                LandNFTErrors::OwnershipNotInitializedForExtraData(element) => element.encode(),
                LandNFTErrors::TransferCallerNotOwnerNorApproved(element) => element.encode(),
                LandNFTErrors::TransferFailed(element) => element.encode(),
                LandNFTErrors::TransferFromIncorrectOwner(element) => element.encode(),
                LandNFTErrors::TransferToNonERC721ReceiverImplementer(element) => element.encode(),
                LandNFTErrors::TransferToZeroAddress(element) => element.encode(),
                LandNFTErrors::URIQueryForNonexistentToken(element) => element.encode(),
                LandNFTErrors::UnauthorizedToMint(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for LandNFTErrors {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                LandNFTErrors::AddressAlreadyClaimed(element) => element.fmt(f),
                LandNFTErrors::ApprovalCallerNotOwnerNorApproved(element) => element.fmt(f),
                LandNFTErrors::ApprovalQueryForNonexistentToken(element) => element.fmt(f),
                LandNFTErrors::ApproveToCaller(element) => element.fmt(f),
                LandNFTErrors::BalanceQueryForZeroAddress(element) => element.fmt(f),
                LandNFTErrors::CannotSetAddressZero(element) => element.fmt(f),
                LandNFTErrors::ComissionOutOfAllowedRange(element) => element.fmt(f),
                LandNFTErrors::InsufficientBalance(element) => element.fmt(f),
                LandNFTErrors::MaxTilesReached(element) => element.fmt(f),
                LandNFTErrors::MintERC2309QuantityExceedsLimit(element) => element.fmt(f),
                LandNFTErrors::MintToZeroAddress(element) => element.fmt(f),
                LandNFTErrors::MintZeroQuantity(element) => element.fmt(f),
                LandNFTErrors::NonExistentTokenURI(element) => element.fmt(f),
                LandNFTErrors::NotAuthorised(element) => element.fmt(f),
                LandNFTErrors::OwnerQueryForNonexistentToken(element) => element.fmt(f),
                LandNFTErrors::OwnershipNotInitializedForExtraData(element) => element.fmt(f),
                LandNFTErrors::TransferCallerNotOwnerNorApproved(element) => element.fmt(f),
                LandNFTErrors::TransferFailed(element) => element.fmt(f),
                LandNFTErrors::TransferFromIncorrectOwner(element) => element.fmt(f),
                LandNFTErrors::TransferToNonERC721ReceiverImplementer(element) => element.fmt(f),
                LandNFTErrors::TransferToZeroAddress(element) => element.fmt(f),
                LandNFTErrors::URIQueryForNonexistentToken(element) => element.fmt(f),
                LandNFTErrors::UnauthorizedToMint(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<AddressAlreadyClaimed> for LandNFTErrors {
        fn from(var: AddressAlreadyClaimed) -> Self {
            LandNFTErrors::AddressAlreadyClaimed(var)
        }
    }
    impl ::std::convert::From<ApprovalCallerNotOwnerNorApproved> for LandNFTErrors {
        fn from(var: ApprovalCallerNotOwnerNorApproved) -> Self {
            LandNFTErrors::ApprovalCallerNotOwnerNorApproved(var)
        }
    }
    impl ::std::convert::From<ApprovalQueryForNonexistentToken> for LandNFTErrors {
        fn from(var: ApprovalQueryForNonexistentToken) -> Self {
            LandNFTErrors::ApprovalQueryForNonexistentToken(var)
        }
    }
    impl ::std::convert::From<ApproveToCaller> for LandNFTErrors {
        fn from(var: ApproveToCaller) -> Self {
            LandNFTErrors::ApproveToCaller(var)
        }
    }
    impl ::std::convert::From<BalanceQueryForZeroAddress> for LandNFTErrors {
        fn from(var: BalanceQueryForZeroAddress) -> Self {
            LandNFTErrors::BalanceQueryForZeroAddress(var)
        }
    }
    impl ::std::convert::From<CannotSetAddressZero> for LandNFTErrors {
        fn from(var: CannotSetAddressZero) -> Self {
            LandNFTErrors::CannotSetAddressZero(var)
        }
    }
    impl ::std::convert::From<ComissionOutOfAllowedRange> for LandNFTErrors {
        fn from(var: ComissionOutOfAllowedRange) -> Self {
            LandNFTErrors::ComissionOutOfAllowedRange(var)
        }
    }
    impl ::std::convert::From<InsufficientBalance> for LandNFTErrors {
        fn from(var: InsufficientBalance) -> Self {
            LandNFTErrors::InsufficientBalance(var)
        }
    }
    impl ::std::convert::From<MaxTilesReached> for LandNFTErrors {
        fn from(var: MaxTilesReached) -> Self {
            LandNFTErrors::MaxTilesReached(var)
        }
    }
    impl ::std::convert::From<MintERC2309QuantityExceedsLimit> for LandNFTErrors {
        fn from(var: MintERC2309QuantityExceedsLimit) -> Self {
            LandNFTErrors::MintERC2309QuantityExceedsLimit(var)
        }
    }
    impl ::std::convert::From<MintToZeroAddress> for LandNFTErrors {
        fn from(var: MintToZeroAddress) -> Self {
            LandNFTErrors::MintToZeroAddress(var)
        }
    }
    impl ::std::convert::From<MintZeroQuantity> for LandNFTErrors {
        fn from(var: MintZeroQuantity) -> Self {
            LandNFTErrors::MintZeroQuantity(var)
        }
    }
    impl ::std::convert::From<NonExistentTokenURI> for LandNFTErrors {
        fn from(var: NonExistentTokenURI) -> Self {
            LandNFTErrors::NonExistentTokenURI(var)
        }
    }
    impl ::std::convert::From<NotAuthorised> for LandNFTErrors {
        fn from(var: NotAuthorised) -> Self {
            LandNFTErrors::NotAuthorised(var)
        }
    }
    impl ::std::convert::From<OwnerQueryForNonexistentToken> for LandNFTErrors {
        fn from(var: OwnerQueryForNonexistentToken) -> Self {
            LandNFTErrors::OwnerQueryForNonexistentToken(var)
        }
    }
    impl ::std::convert::From<OwnershipNotInitializedForExtraData> for LandNFTErrors {
        fn from(var: OwnershipNotInitializedForExtraData) -> Self {
            LandNFTErrors::OwnershipNotInitializedForExtraData(var)
        }
    }
    impl ::std::convert::From<TransferCallerNotOwnerNorApproved> for LandNFTErrors {
        fn from(var: TransferCallerNotOwnerNorApproved) -> Self {
            LandNFTErrors::TransferCallerNotOwnerNorApproved(var)
        }
    }
    impl ::std::convert::From<TransferFailed> for LandNFTErrors {
        fn from(var: TransferFailed) -> Self {
            LandNFTErrors::TransferFailed(var)
        }
    }
    impl ::std::convert::From<TransferFromIncorrectOwner> for LandNFTErrors {
        fn from(var: TransferFromIncorrectOwner) -> Self {
            LandNFTErrors::TransferFromIncorrectOwner(var)
        }
    }
    impl ::std::convert::From<TransferToNonERC721ReceiverImplementer> for LandNFTErrors {
        fn from(var: TransferToNonERC721ReceiverImplementer) -> Self {
            LandNFTErrors::TransferToNonERC721ReceiverImplementer(var)
        }
    }
    impl ::std::convert::From<TransferToZeroAddress> for LandNFTErrors {
        fn from(var: TransferToZeroAddress) -> Self {
            LandNFTErrors::TransferToZeroAddress(var)
        }
    }
    impl ::std::convert::From<URIQueryForNonexistentToken> for LandNFTErrors {
        fn from(var: URIQueryForNonexistentToken) -> Self {
            LandNFTErrors::URIQueryForNonexistentToken(var)
        }
    }
    impl ::std::convert::From<UnauthorizedToMint> for LandNFTErrors {
        fn from(var: UnauthorizedToMint) -> Self {
            LandNFTErrors::UnauthorizedToMint(var)
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
    #[ethevent(name = "AdminChanged", abi = "AdminChanged(address,address)")]
    pub struct AdminChangedFilter {
        #[ethevent(indexed)]
        pub new_admin: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub old_admin: ethers::core::types::Address,
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
        name = "CommissionRateChanged",
        abi = "CommissionRateChanged(uint256,uint256)"
    )]
    pub struct CommissionRateChangedFilter {
        #[ethevent(indexed)]
        pub new_commission: ethers::core::types::U256,
        #[ethevent(indexed)]
        pub old_commission: ethers::core::types::U256,
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
    #[ethevent(name = "ContractPaused", abi = "ContractPaused(bool)")]
    pub struct ContractPausedFilter {
        #[ethevent(indexed)]
        pub paused: bool,
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
    #[ethevent(name = "ContractUnpaused", abi = "ContractUnpaused(bool)")]
    pub struct ContractUnpausedFilter {
        #[ethevent(indexed)]
        pub paused: bool,
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
    #[ethevent(name = "DevFundChanged", abi = "DevFundChanged(address,address)")]
    pub struct DevFundChangedFilter {
        #[ethevent(indexed)]
        pub new_dev_fund: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub old_dev_fund: ethers::core::types::Address,
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
    #[ethevent(name = "LandBankChanged", abi = "LandBankChanged(address,address)")]
    pub struct LandBankChangedFilter {
        #[ethevent(indexed)]
        pub new_land_bank: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub old_land_bank: ethers::core::types::Address,
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
        name = "LandSold",
        abi = "LandSold(address,((uint256,uint256),(uint256,uint256),(uint256,uint256),(uint256,uint256))[])"
    )]
    pub struct LandSoldFilter {
        #[ethevent(indexed)]
        pub buyer: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub region: ethers::core::types::H256,
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
        name = "OwnershipTransferred",
        abi = "OwnershipTransferred(address,address)"
    )]
    pub struct OwnershipTransferredFilter {
        #[ethevent(indexed)]
        pub previous_owner: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub new_owner: ethers::core::types::Address,
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
    #[ethevent(name = "Paused", abi = "Paused(address)")]
    pub struct PausedFilter {
        pub account: ethers::core::types::Address,
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
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethevent(name = "Unpaused", abi = "Unpaused(address)")]
    pub struct UnpausedFilter {
        pub account: ethers::core::types::Address,
    }
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum LandNFTEvents {
        AdminChangedFilter(AdminChangedFilter),
        ApprovalFilter(ApprovalFilter),
        ApprovalForAllFilter(ApprovalForAllFilter),
        CommissionRateChangedFilter(CommissionRateChangedFilter),
        ConsecutiveTransferFilter(ConsecutiveTransferFilter),
        ContractPausedFilter(ContractPausedFilter),
        ContractUnpausedFilter(ContractUnpausedFilter),
        DevFundChangedFilter(DevFundChangedFilter),
        LandBankChangedFilter(LandBankChangedFilter),
        LandSoldFilter(LandSoldFilter),
        OwnershipTransferredFilter(OwnershipTransferredFilter),
        PausedFilter(PausedFilter),
        TransferFilter(TransferFilter),
        UnpausedFilter(UnpausedFilter),
    }
    impl ethers::contract::EthLogDecode for LandNFTEvents {
        fn decode_log(
            log: &ethers::core::abi::RawLog,
        ) -> ::std::result::Result<Self, ethers::core::abi::Error>
        where
            Self: Sized,
        {
            if let Ok(decoded) = AdminChangedFilter::decode_log(log) {
                return Ok(LandNFTEvents::AdminChangedFilter(decoded));
            }
            if let Ok(decoded) = ApprovalFilter::decode_log(log) {
                return Ok(LandNFTEvents::ApprovalFilter(decoded));
            }
            if let Ok(decoded) = ApprovalForAllFilter::decode_log(log) {
                return Ok(LandNFTEvents::ApprovalForAllFilter(decoded));
            }
            if let Ok(decoded) = CommissionRateChangedFilter::decode_log(log) {
                return Ok(LandNFTEvents::CommissionRateChangedFilter(decoded));
            }
            if let Ok(decoded) = ConsecutiveTransferFilter::decode_log(log) {
                return Ok(LandNFTEvents::ConsecutiveTransferFilter(decoded));
            }
            if let Ok(decoded) = ContractPausedFilter::decode_log(log) {
                return Ok(LandNFTEvents::ContractPausedFilter(decoded));
            }
            if let Ok(decoded) = ContractUnpausedFilter::decode_log(log) {
                return Ok(LandNFTEvents::ContractUnpausedFilter(decoded));
            }
            if let Ok(decoded) = DevFundChangedFilter::decode_log(log) {
                return Ok(LandNFTEvents::DevFundChangedFilter(decoded));
            }
            if let Ok(decoded) = LandBankChangedFilter::decode_log(log) {
                return Ok(LandNFTEvents::LandBankChangedFilter(decoded));
            }
            if let Ok(decoded) = LandSoldFilter::decode_log(log) {
                return Ok(LandNFTEvents::LandSoldFilter(decoded));
            }
            if let Ok(decoded) = OwnershipTransferredFilter::decode_log(log) {
                return Ok(LandNFTEvents::OwnershipTransferredFilter(decoded));
            }
            if let Ok(decoded) = PausedFilter::decode_log(log) {
                return Ok(LandNFTEvents::PausedFilter(decoded));
            }
            if let Ok(decoded) = TransferFilter::decode_log(log) {
                return Ok(LandNFTEvents::TransferFilter(decoded));
            }
            if let Ok(decoded) = UnpausedFilter::decode_log(log) {
                return Ok(LandNFTEvents::UnpausedFilter(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::std::fmt::Display for LandNFTEvents {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                LandNFTEvents::AdminChangedFilter(element) => element.fmt(f),
                LandNFTEvents::ApprovalFilter(element) => element.fmt(f),
                LandNFTEvents::ApprovalForAllFilter(element) => element.fmt(f),
                LandNFTEvents::CommissionRateChangedFilter(element) => element.fmt(f),
                LandNFTEvents::ConsecutiveTransferFilter(element) => element.fmt(f),
                LandNFTEvents::ContractPausedFilter(element) => element.fmt(f),
                LandNFTEvents::ContractUnpausedFilter(element) => element.fmt(f),
                LandNFTEvents::DevFundChangedFilter(element) => element.fmt(f),
                LandNFTEvents::LandBankChangedFilter(element) => element.fmt(f),
                LandNFTEvents::LandSoldFilter(element) => element.fmt(f),
                LandNFTEvents::OwnershipTransferredFilter(element) => element.fmt(f),
                LandNFTEvents::PausedFilter(element) => element.fmt(f),
                LandNFTEvents::TransferFilter(element) => element.fmt(f),
                LandNFTEvents::UnpausedFilter(element) => element.fmt(f),
            }
        }
    }
    #[doc = "Container type for all input parameters for the `MAX_TILE_NUM` function with signature `MAX_TILE_NUM()` and selector `[245, 247, 88, 27]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "MAX_TILE_NUM", abi = "MAX_TILE_NUM()")]
    pub struct MaxTileNumCall;
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
    #[doc = "Container type for all input parameters for the `admin` function with signature `admin()` and selector `[248, 81, 164, 64]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "admin", abi = "admin()")]
    pub struct AdminCall;
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
    #[doc = "Container type for all input parameters for the `baseURI` function with signature `baseURI()` and selector `[108, 3, 96, 235]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "baseURI", abi = "baseURI()")]
    pub struct BaseURICall;
    #[doc = "Container type for all input parameters for the `claimSpecialArea` function with signature `claimSpecialArea(bytes32[],((uint256,uint256),(uint256,uint256),(uint256,uint256),(uint256,uint256)))` and selector `[30, 66, 65, 42]`"]
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
        name = "claimSpecialArea",
        abi = "claimSpecialArea(bytes32[],((uint256,uint256),(uint256,uint256),(uint256,uint256),(uint256,uint256)))"
    )]
    pub struct ClaimSpecialAreaCall {
        pub merkle_proof: ::std::vec::Vec<[u8; 32]>,
        pub region: Pixel,
    }
    #[doc = "Container type for all input parameters for the `commissionRate` function with signature `commissionRate()` and selector `[94, 161, 214, 248]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "commissionRate", abi = "commissionRate()")]
    pub struct CommissionRateCall;
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
    #[doc = "Container type for all input parameters for the `firstOwners` function with signature `firstOwners(uint256)` and selector `[239, 175, 240, 60]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "firstOwners", abi = "firstOwners(uint256)")]
    pub struct FirstOwnersCall(pub ethers::core::types::U256);
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
    #[doc = "Container type for all input parameters for the `getTokenPrice` function with signature `getTokenPrice(uint256)` and selector `[196, 87, 251, 55]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "getTokenPrice", abi = "getTokenPrice(uint256)")]
    pub struct GetTokenPriceCall {
        pub amount: ethers::core::types::U256,
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
    #[doc = "Container type for all input parameters for the `isOwned` function with signature `isOwned(uint256)` and selector `[89, 159, 104, 156]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "isOwned", abi = "isOwned(uint256)")]
    pub struct IsOwnedCall(pub ethers::core::types::U256);
    #[doc = "Container type for all input parameters for the `landBank` function with signature `landBank()` and selector `[239, 211, 41, 115]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "landBank", abi = "landBank()")]
    pub struct LandBankCall;
    #[doc = "Container type for all input parameters for the `merkleRoot` function with signature `merkleRoot()` and selector `[46, 180, 167, 171]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "merkleRoot", abi = "merkleRoot()")]
    pub struct MerkleRootCall;
    #[doc = "Container type for all input parameters for the `mint` function with signature `mint(((uint256,uint256),(uint256,uint256),(uint256,uint256),(uint256,uint256))[],uint256)` and selector `[1, 154, 229, 153]`"]
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
        name = "mint",
        abi = "mint(((uint256,uint256),(uint256,uint256),(uint256,uint256),(uint256,uint256))[],uint256)"
    )]
    pub struct MintCall {
        pub region: ::std::vec::Vec<Pixel>,
        pub rio_amount: ethers::core::types::U256,
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
    #[doc = "Container type for all input parameters for the `nextId` function with signature `nextId()` and selector `[97, 184, 206, 140]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "nextId", abi = "nextId()")]
    pub struct NextIdCall;
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
    #[doc = "Container type for all input parameters for the `pause` function with signature `pause()` and selector `[132, 86, 203, 89]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "pause", abi = "pause()")]
    pub struct PauseCall;
    #[doc = "Container type for all input parameters for the `paused` function with signature `paused()` and selector `[92, 151, 90, 187]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "paused", abi = "paused()")]
    pub struct PausedCall;
    #[doc = "Container type for all input parameters for the `price` function with signature `price()` and selector `[160, 53, 177, 254]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "price", abi = "price()")]
    pub struct PriceCall;
    #[doc = "Container type for all input parameters for the `rareClaimedPeriod` function with signature `rareClaimedPeriod()` and selector `[164, 235, 203, 174]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "rareClaimedPeriod", abi = "rareClaimedPeriod()")]
    pub struct RareClaimedPeriodCall;
    #[doc = "Container type for all input parameters for the `renounceOwnership` function with signature `renounceOwnership()` and selector `[113, 80, 24, 166]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "renounceOwnership", abi = "renounceOwnership()")]
    pub struct RenounceOwnershipCall;
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
    #[doc = "Container type for all input parameters for the `setAdmin` function with signature `setAdmin(address)` and selector `[112, 75, 108, 2]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "setAdmin", abi = "setAdmin(address)")]
    pub struct SetAdminCall {
        pub new_admin: ethers::core::types::Address,
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
    #[doc = "Container type for all input parameters for the `setBaseURI` function with signature `setBaseURI(string)` and selector `[85, 248, 4, 179]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "setBaseURI", abi = "setBaseURI(string)")]
    pub struct SetBaseURICall {
        pub base_uri: String,
    }
    #[doc = "Container type for all input parameters for the `setCommissionRate` function with signature `setCommissionRate(uint256)` and selector `[25, 250, 200, 253]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "setCommissionRate", abi = "setCommissionRate(uint256)")]
    pub struct SetCommissionRateCall {
        pub commission_rate: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `setDevFund` function with signature `setDevFund(address)` and selector `[174, 77, 185, 25]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "setDevFund", abi = "setDevFund(address)")]
    pub struct SetDevFundCall {
        pub dev_fund: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `setLandBank` function with signature `setLandBank(address)` and selector `[240, 100, 13, 149]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "setLandBank", abi = "setLandBank(address)")]
    pub struct SetLandBankCall {
        pub land_bank: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `setMerkleRoot` function with signature `setMerkleRoot(bytes32)` and selector `[124, 182, 71, 89]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "setMerkleRoot", abi = "setMerkleRoot(bytes32)")]
    pub struct SetMerkleRootCall {
        pub root: [u8; 32],
    }
    #[doc = "Container type for all input parameters for the `setPrice` function with signature `setPrice(uint256)` and selector `[145, 183, 245, 237]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "setPrice", abi = "setPrice(uint256)")]
    pub struct SetPriceCall {
        pub price: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `setSpecialArea` function with signature `setSpecialArea((uint256,uint256),(uint256,uint256),(uint256,uint256),(uint256,uint256))` and selector `[254, 144, 159, 36]`"]
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
        name = "setSpecialArea",
        abi = "setSpecialArea((uint256,uint256),(uint256,uint256),(uint256,uint256),(uint256,uint256))"
    )]
    pub struct SetSpecialAreaCall {
        pub a: Coordonate,
        pub b: Coordonate,
        pub c: Coordonate,
        pub d: Coordonate,
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
    #[doc = "Container type for all input parameters for the `tilesBought` function with signature `tilesBought()` and selector `[196, 39, 226, 181]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "tilesBought", abi = "tilesBought()")]
    pub struct TilesBoughtCall;
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
    #[doc = "Container type for all input parameters for the `totalTileNum` function with signature `totalTileNum()` and selector `[97, 139, 42, 221]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "totalTileNum", abi = "totalTileNum()")]
    pub struct TotalTileNumCall;
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
    #[doc = "Container type for all input parameters for the `transferOwnership` function with signature `transferOwnership(address)` and selector `[242, 253, 227, 139]`"]
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
    #[doc = "Container type for all input parameters for the `unpause` function with signature `unpause()` and selector `[63, 75, 168, 58]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "unpause", abi = "unpause()")]
    pub struct UnpauseCall;
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum LandNFTCalls {
        MaxTileNum(MaxTileNumCall),
        Weth(WethCall),
        Admin(AdminCall),
        Approve(ApproveCall),
        BalanceOf(BalanceOfCall),
        BaseURI(BaseURICall),
        ClaimSpecialArea(ClaimSpecialAreaCall),
        CommissionRate(CommissionRateCall),
        DevFund(DevFundCall),
        FirstOwners(FirstOwnersCall),
        GetAmountOutMin(GetAmountOutMinCall),
        GetApproved(GetApprovedCall),
        GetTokenPrice(GetTokenPriceCall),
        IsApprovedForAll(IsApprovedForAllCall),
        IsOwned(IsOwnedCall),
        LandBank(LandBankCall),
        MerkleRoot(MerkleRootCall),
        Mint(MintCall),
        Name(NameCall),
        NextId(NextIdCall),
        Owner(OwnerCall),
        OwnerOf(OwnerOfCall),
        Pause(PauseCall),
        Paused(PausedCall),
        Price(PriceCall),
        RareClaimedPeriod(RareClaimedPeriodCall),
        RenounceOwnership(RenounceOwnershipCall),
        SafeTransferFrom(SafeTransferFromCall),
        SafeTransferFromWithFromAndToAndData(SafeTransferFromWithFromAndToAndDataCall),
        SetAdmin(SetAdminCall),
        SetApprovalForAll(SetApprovalForAllCall),
        SetBaseURI(SetBaseURICall),
        SetCommissionRate(SetCommissionRateCall),
        SetDevFund(SetDevFundCall),
        SetLandBank(SetLandBankCall),
        SetMerkleRoot(SetMerkleRootCall),
        SetPrice(SetPriceCall),
        SetSpecialArea(SetSpecialAreaCall),
        SupportsInterface(SupportsInterfaceCall),
        Symbol(SymbolCall),
        TilesBought(TilesBoughtCall),
        TokenURI(TokenURICall),
        TotalSupply(TotalSupplyCall),
        TotalTileNum(TotalTileNumCall),
        TransferFrom(TransferFromCall),
        TransferOwnership(TransferOwnershipCall),
        Unpause(UnpauseCall),
    }
    impl ethers::core::abi::AbiDecode for LandNFTCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::std::result::Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <MaxTileNumCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(LandNFTCalls::MaxTileNum(decoded));
            }
            if let Ok(decoded) = <WethCall as ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(LandNFTCalls::Weth(decoded));
            }
            if let Ok(decoded) = <AdminCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(LandNFTCalls::Admin(decoded));
            }
            if let Ok(decoded) =
                <ApproveCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(LandNFTCalls::Approve(decoded));
            }
            if let Ok(decoded) =
                <BalanceOfCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(LandNFTCalls::BalanceOf(decoded));
            }
            if let Ok(decoded) =
                <BaseURICall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(LandNFTCalls::BaseURI(decoded));
            }
            if let Ok(decoded) =
                <ClaimSpecialAreaCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(LandNFTCalls::ClaimSpecialArea(decoded));
            }
            if let Ok(decoded) =
                <CommissionRateCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(LandNFTCalls::CommissionRate(decoded));
            }
            if let Ok(decoded) =
                <DevFundCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(LandNFTCalls::DevFund(decoded));
            }
            if let Ok(decoded) =
                <FirstOwnersCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(LandNFTCalls::FirstOwners(decoded));
            }
            if let Ok(decoded) =
                <GetAmountOutMinCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(LandNFTCalls::GetAmountOutMin(decoded));
            }
            if let Ok(decoded) =
                <GetApprovedCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(LandNFTCalls::GetApproved(decoded));
            }
            if let Ok(decoded) =
                <GetTokenPriceCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(LandNFTCalls::GetTokenPrice(decoded));
            }
            if let Ok(decoded) =
                <IsApprovedForAllCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(LandNFTCalls::IsApprovedForAll(decoded));
            }
            if let Ok(decoded) =
                <IsOwnedCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(LandNFTCalls::IsOwned(decoded));
            }
            if let Ok(decoded) =
                <LandBankCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(LandNFTCalls::LandBank(decoded));
            }
            if let Ok(decoded) =
                <MerkleRootCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(LandNFTCalls::MerkleRoot(decoded));
            }
            if let Ok(decoded) = <MintCall as ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(LandNFTCalls::Mint(decoded));
            }
            if let Ok(decoded) = <NameCall as ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(LandNFTCalls::Name(decoded));
            }
            if let Ok(decoded) = <NextIdCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(LandNFTCalls::NextId(decoded));
            }
            if let Ok(decoded) = <OwnerCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(LandNFTCalls::Owner(decoded));
            }
            if let Ok(decoded) =
                <OwnerOfCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(LandNFTCalls::OwnerOf(decoded));
            }
            if let Ok(decoded) = <PauseCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(LandNFTCalls::Pause(decoded));
            }
            if let Ok(decoded) = <PausedCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(LandNFTCalls::Paused(decoded));
            }
            if let Ok(decoded) = <PriceCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(LandNFTCalls::Price(decoded));
            }
            if let Ok(decoded) =
                <RareClaimedPeriodCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(LandNFTCalls::RareClaimedPeriod(decoded));
            }
            if let Ok(decoded) =
                <RenounceOwnershipCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(LandNFTCalls::RenounceOwnership(decoded));
            }
            if let Ok(decoded) =
                <SafeTransferFromCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(LandNFTCalls::SafeTransferFrom(decoded));
            }
            if let Ok(decoded) =
                <SafeTransferFromWithFromAndToAndDataCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(LandNFTCalls::SafeTransferFromWithFromAndToAndData(decoded));
            }
            if let Ok(decoded) =
                <SetAdminCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(LandNFTCalls::SetAdmin(decoded));
            }
            if let Ok(decoded) =
                <SetApprovalForAllCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(LandNFTCalls::SetApprovalForAll(decoded));
            }
            if let Ok(decoded) =
                <SetBaseURICall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(LandNFTCalls::SetBaseURI(decoded));
            }
            if let Ok(decoded) =
                <SetCommissionRateCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(LandNFTCalls::SetCommissionRate(decoded));
            }
            if let Ok(decoded) =
                <SetDevFundCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(LandNFTCalls::SetDevFund(decoded));
            }
            if let Ok(decoded) =
                <SetLandBankCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(LandNFTCalls::SetLandBank(decoded));
            }
            if let Ok(decoded) =
                <SetMerkleRootCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(LandNFTCalls::SetMerkleRoot(decoded));
            }
            if let Ok(decoded) =
                <SetPriceCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(LandNFTCalls::SetPrice(decoded));
            }
            if let Ok(decoded) =
                <SetSpecialAreaCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(LandNFTCalls::SetSpecialArea(decoded));
            }
            if let Ok(decoded) =
                <SupportsInterfaceCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(LandNFTCalls::SupportsInterface(decoded));
            }
            if let Ok(decoded) = <SymbolCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(LandNFTCalls::Symbol(decoded));
            }
            if let Ok(decoded) =
                <TilesBoughtCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(LandNFTCalls::TilesBought(decoded));
            }
            if let Ok(decoded) =
                <TokenURICall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(LandNFTCalls::TokenURI(decoded));
            }
            if let Ok(decoded) =
                <TotalSupplyCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(LandNFTCalls::TotalSupply(decoded));
            }
            if let Ok(decoded) =
                <TotalTileNumCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(LandNFTCalls::TotalTileNum(decoded));
            }
            if let Ok(decoded) =
                <TransferFromCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(LandNFTCalls::TransferFrom(decoded));
            }
            if let Ok(decoded) =
                <TransferOwnershipCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(LandNFTCalls::TransferOwnership(decoded));
            }
            if let Ok(decoded) =
                <UnpauseCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(LandNFTCalls::Unpause(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for LandNFTCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                LandNFTCalls::MaxTileNum(element) => element.encode(),
                LandNFTCalls::Weth(element) => element.encode(),
                LandNFTCalls::Admin(element) => element.encode(),
                LandNFTCalls::Approve(element) => element.encode(),
                LandNFTCalls::BalanceOf(element) => element.encode(),
                LandNFTCalls::BaseURI(element) => element.encode(),
                LandNFTCalls::ClaimSpecialArea(element) => element.encode(),
                LandNFTCalls::CommissionRate(element) => element.encode(),
                LandNFTCalls::DevFund(element) => element.encode(),
                LandNFTCalls::FirstOwners(element) => element.encode(),
                LandNFTCalls::GetAmountOutMin(element) => element.encode(),
                LandNFTCalls::GetApproved(element) => element.encode(),
                LandNFTCalls::GetTokenPrice(element) => element.encode(),
                LandNFTCalls::IsApprovedForAll(element) => element.encode(),
                LandNFTCalls::IsOwned(element) => element.encode(),
                LandNFTCalls::LandBank(element) => element.encode(),
                LandNFTCalls::MerkleRoot(element) => element.encode(),
                LandNFTCalls::Mint(element) => element.encode(),
                LandNFTCalls::Name(element) => element.encode(),
                LandNFTCalls::NextId(element) => element.encode(),
                LandNFTCalls::Owner(element) => element.encode(),
                LandNFTCalls::OwnerOf(element) => element.encode(),
                LandNFTCalls::Pause(element) => element.encode(),
                LandNFTCalls::Paused(element) => element.encode(),
                LandNFTCalls::Price(element) => element.encode(),
                LandNFTCalls::RareClaimedPeriod(element) => element.encode(),
                LandNFTCalls::RenounceOwnership(element) => element.encode(),
                LandNFTCalls::SafeTransferFrom(element) => element.encode(),
                LandNFTCalls::SafeTransferFromWithFromAndToAndData(element) => element.encode(),
                LandNFTCalls::SetAdmin(element) => element.encode(),
                LandNFTCalls::SetApprovalForAll(element) => element.encode(),
                LandNFTCalls::SetBaseURI(element) => element.encode(),
                LandNFTCalls::SetCommissionRate(element) => element.encode(),
                LandNFTCalls::SetDevFund(element) => element.encode(),
                LandNFTCalls::SetLandBank(element) => element.encode(),
                LandNFTCalls::SetMerkleRoot(element) => element.encode(),
                LandNFTCalls::SetPrice(element) => element.encode(),
                LandNFTCalls::SetSpecialArea(element) => element.encode(),
                LandNFTCalls::SupportsInterface(element) => element.encode(),
                LandNFTCalls::Symbol(element) => element.encode(),
                LandNFTCalls::TilesBought(element) => element.encode(),
                LandNFTCalls::TokenURI(element) => element.encode(),
                LandNFTCalls::TotalSupply(element) => element.encode(),
                LandNFTCalls::TotalTileNum(element) => element.encode(),
                LandNFTCalls::TransferFrom(element) => element.encode(),
                LandNFTCalls::TransferOwnership(element) => element.encode(),
                LandNFTCalls::Unpause(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for LandNFTCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                LandNFTCalls::MaxTileNum(element) => element.fmt(f),
                LandNFTCalls::Weth(element) => element.fmt(f),
                LandNFTCalls::Admin(element) => element.fmt(f),
                LandNFTCalls::Approve(element) => element.fmt(f),
                LandNFTCalls::BalanceOf(element) => element.fmt(f),
                LandNFTCalls::BaseURI(element) => element.fmt(f),
                LandNFTCalls::ClaimSpecialArea(element) => element.fmt(f),
                LandNFTCalls::CommissionRate(element) => element.fmt(f),
                LandNFTCalls::DevFund(element) => element.fmt(f),
                LandNFTCalls::FirstOwners(element) => element.fmt(f),
                LandNFTCalls::GetAmountOutMin(element) => element.fmt(f),
                LandNFTCalls::GetApproved(element) => element.fmt(f),
                LandNFTCalls::GetTokenPrice(element) => element.fmt(f),
                LandNFTCalls::IsApprovedForAll(element) => element.fmt(f),
                LandNFTCalls::IsOwned(element) => element.fmt(f),
                LandNFTCalls::LandBank(element) => element.fmt(f),
                LandNFTCalls::MerkleRoot(element) => element.fmt(f),
                LandNFTCalls::Mint(element) => element.fmt(f),
                LandNFTCalls::Name(element) => element.fmt(f),
                LandNFTCalls::NextId(element) => element.fmt(f),
                LandNFTCalls::Owner(element) => element.fmt(f),
                LandNFTCalls::OwnerOf(element) => element.fmt(f),
                LandNFTCalls::Pause(element) => element.fmt(f),
                LandNFTCalls::Paused(element) => element.fmt(f),
                LandNFTCalls::Price(element) => element.fmt(f),
                LandNFTCalls::RareClaimedPeriod(element) => element.fmt(f),
                LandNFTCalls::RenounceOwnership(element) => element.fmt(f),
                LandNFTCalls::SafeTransferFrom(element) => element.fmt(f),
                LandNFTCalls::SafeTransferFromWithFromAndToAndData(element) => element.fmt(f),
                LandNFTCalls::SetAdmin(element) => element.fmt(f),
                LandNFTCalls::SetApprovalForAll(element) => element.fmt(f),
                LandNFTCalls::SetBaseURI(element) => element.fmt(f),
                LandNFTCalls::SetCommissionRate(element) => element.fmt(f),
                LandNFTCalls::SetDevFund(element) => element.fmt(f),
                LandNFTCalls::SetLandBank(element) => element.fmt(f),
                LandNFTCalls::SetMerkleRoot(element) => element.fmt(f),
                LandNFTCalls::SetPrice(element) => element.fmt(f),
                LandNFTCalls::SetSpecialArea(element) => element.fmt(f),
                LandNFTCalls::SupportsInterface(element) => element.fmt(f),
                LandNFTCalls::Symbol(element) => element.fmt(f),
                LandNFTCalls::TilesBought(element) => element.fmt(f),
                LandNFTCalls::TokenURI(element) => element.fmt(f),
                LandNFTCalls::TotalSupply(element) => element.fmt(f),
                LandNFTCalls::TotalTileNum(element) => element.fmt(f),
                LandNFTCalls::TransferFrom(element) => element.fmt(f),
                LandNFTCalls::TransferOwnership(element) => element.fmt(f),
                LandNFTCalls::Unpause(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<MaxTileNumCall> for LandNFTCalls {
        fn from(var: MaxTileNumCall) -> Self {
            LandNFTCalls::MaxTileNum(var)
        }
    }
    impl ::std::convert::From<WethCall> for LandNFTCalls {
        fn from(var: WethCall) -> Self {
            LandNFTCalls::Weth(var)
        }
    }
    impl ::std::convert::From<AdminCall> for LandNFTCalls {
        fn from(var: AdminCall) -> Self {
            LandNFTCalls::Admin(var)
        }
    }
    impl ::std::convert::From<ApproveCall> for LandNFTCalls {
        fn from(var: ApproveCall) -> Self {
            LandNFTCalls::Approve(var)
        }
    }
    impl ::std::convert::From<BalanceOfCall> for LandNFTCalls {
        fn from(var: BalanceOfCall) -> Self {
            LandNFTCalls::BalanceOf(var)
        }
    }
    impl ::std::convert::From<BaseURICall> for LandNFTCalls {
        fn from(var: BaseURICall) -> Self {
            LandNFTCalls::BaseURI(var)
        }
    }
    impl ::std::convert::From<ClaimSpecialAreaCall> for LandNFTCalls {
        fn from(var: ClaimSpecialAreaCall) -> Self {
            LandNFTCalls::ClaimSpecialArea(var)
        }
    }
    impl ::std::convert::From<CommissionRateCall> for LandNFTCalls {
        fn from(var: CommissionRateCall) -> Self {
            LandNFTCalls::CommissionRate(var)
        }
    }
    impl ::std::convert::From<DevFundCall> for LandNFTCalls {
        fn from(var: DevFundCall) -> Self {
            LandNFTCalls::DevFund(var)
        }
    }
    impl ::std::convert::From<FirstOwnersCall> for LandNFTCalls {
        fn from(var: FirstOwnersCall) -> Self {
            LandNFTCalls::FirstOwners(var)
        }
    }
    impl ::std::convert::From<GetAmountOutMinCall> for LandNFTCalls {
        fn from(var: GetAmountOutMinCall) -> Self {
            LandNFTCalls::GetAmountOutMin(var)
        }
    }
    impl ::std::convert::From<GetApprovedCall> for LandNFTCalls {
        fn from(var: GetApprovedCall) -> Self {
            LandNFTCalls::GetApproved(var)
        }
    }
    impl ::std::convert::From<GetTokenPriceCall> for LandNFTCalls {
        fn from(var: GetTokenPriceCall) -> Self {
            LandNFTCalls::GetTokenPrice(var)
        }
    }
    impl ::std::convert::From<IsApprovedForAllCall> for LandNFTCalls {
        fn from(var: IsApprovedForAllCall) -> Self {
            LandNFTCalls::IsApprovedForAll(var)
        }
    }
    impl ::std::convert::From<IsOwnedCall> for LandNFTCalls {
        fn from(var: IsOwnedCall) -> Self {
            LandNFTCalls::IsOwned(var)
        }
    }
    impl ::std::convert::From<LandBankCall> for LandNFTCalls {
        fn from(var: LandBankCall) -> Self {
            LandNFTCalls::LandBank(var)
        }
    }
    impl ::std::convert::From<MerkleRootCall> for LandNFTCalls {
        fn from(var: MerkleRootCall) -> Self {
            LandNFTCalls::MerkleRoot(var)
        }
    }
    impl ::std::convert::From<MintCall> for LandNFTCalls {
        fn from(var: MintCall) -> Self {
            LandNFTCalls::Mint(var)
        }
    }
    impl ::std::convert::From<NameCall> for LandNFTCalls {
        fn from(var: NameCall) -> Self {
            LandNFTCalls::Name(var)
        }
    }
    impl ::std::convert::From<NextIdCall> for LandNFTCalls {
        fn from(var: NextIdCall) -> Self {
            LandNFTCalls::NextId(var)
        }
    }
    impl ::std::convert::From<OwnerCall> for LandNFTCalls {
        fn from(var: OwnerCall) -> Self {
            LandNFTCalls::Owner(var)
        }
    }
    impl ::std::convert::From<OwnerOfCall> for LandNFTCalls {
        fn from(var: OwnerOfCall) -> Self {
            LandNFTCalls::OwnerOf(var)
        }
    }
    impl ::std::convert::From<PauseCall> for LandNFTCalls {
        fn from(var: PauseCall) -> Self {
            LandNFTCalls::Pause(var)
        }
    }
    impl ::std::convert::From<PausedCall> for LandNFTCalls {
        fn from(var: PausedCall) -> Self {
            LandNFTCalls::Paused(var)
        }
    }
    impl ::std::convert::From<PriceCall> for LandNFTCalls {
        fn from(var: PriceCall) -> Self {
            LandNFTCalls::Price(var)
        }
    }
    impl ::std::convert::From<RareClaimedPeriodCall> for LandNFTCalls {
        fn from(var: RareClaimedPeriodCall) -> Self {
            LandNFTCalls::RareClaimedPeriod(var)
        }
    }
    impl ::std::convert::From<RenounceOwnershipCall> for LandNFTCalls {
        fn from(var: RenounceOwnershipCall) -> Self {
            LandNFTCalls::RenounceOwnership(var)
        }
    }
    impl ::std::convert::From<SafeTransferFromCall> for LandNFTCalls {
        fn from(var: SafeTransferFromCall) -> Self {
            LandNFTCalls::SafeTransferFrom(var)
        }
    }
    impl ::std::convert::From<SafeTransferFromWithFromAndToAndDataCall> for LandNFTCalls {
        fn from(var: SafeTransferFromWithFromAndToAndDataCall) -> Self {
            LandNFTCalls::SafeTransferFromWithFromAndToAndData(var)
        }
    }
    impl ::std::convert::From<SetAdminCall> for LandNFTCalls {
        fn from(var: SetAdminCall) -> Self {
            LandNFTCalls::SetAdmin(var)
        }
    }
    impl ::std::convert::From<SetApprovalForAllCall> for LandNFTCalls {
        fn from(var: SetApprovalForAllCall) -> Self {
            LandNFTCalls::SetApprovalForAll(var)
        }
    }
    impl ::std::convert::From<SetBaseURICall> for LandNFTCalls {
        fn from(var: SetBaseURICall) -> Self {
            LandNFTCalls::SetBaseURI(var)
        }
    }
    impl ::std::convert::From<SetCommissionRateCall> for LandNFTCalls {
        fn from(var: SetCommissionRateCall) -> Self {
            LandNFTCalls::SetCommissionRate(var)
        }
    }
    impl ::std::convert::From<SetDevFundCall> for LandNFTCalls {
        fn from(var: SetDevFundCall) -> Self {
            LandNFTCalls::SetDevFund(var)
        }
    }
    impl ::std::convert::From<SetLandBankCall> for LandNFTCalls {
        fn from(var: SetLandBankCall) -> Self {
            LandNFTCalls::SetLandBank(var)
        }
    }
    impl ::std::convert::From<SetMerkleRootCall> for LandNFTCalls {
        fn from(var: SetMerkleRootCall) -> Self {
            LandNFTCalls::SetMerkleRoot(var)
        }
    }
    impl ::std::convert::From<SetPriceCall> for LandNFTCalls {
        fn from(var: SetPriceCall) -> Self {
            LandNFTCalls::SetPrice(var)
        }
    }
    impl ::std::convert::From<SetSpecialAreaCall> for LandNFTCalls {
        fn from(var: SetSpecialAreaCall) -> Self {
            LandNFTCalls::SetSpecialArea(var)
        }
    }
    impl ::std::convert::From<SupportsInterfaceCall> for LandNFTCalls {
        fn from(var: SupportsInterfaceCall) -> Self {
            LandNFTCalls::SupportsInterface(var)
        }
    }
    impl ::std::convert::From<SymbolCall> for LandNFTCalls {
        fn from(var: SymbolCall) -> Self {
            LandNFTCalls::Symbol(var)
        }
    }
    impl ::std::convert::From<TilesBoughtCall> for LandNFTCalls {
        fn from(var: TilesBoughtCall) -> Self {
            LandNFTCalls::TilesBought(var)
        }
    }
    impl ::std::convert::From<TokenURICall> for LandNFTCalls {
        fn from(var: TokenURICall) -> Self {
            LandNFTCalls::TokenURI(var)
        }
    }
    impl ::std::convert::From<TotalSupplyCall> for LandNFTCalls {
        fn from(var: TotalSupplyCall) -> Self {
            LandNFTCalls::TotalSupply(var)
        }
    }
    impl ::std::convert::From<TotalTileNumCall> for LandNFTCalls {
        fn from(var: TotalTileNumCall) -> Self {
            LandNFTCalls::TotalTileNum(var)
        }
    }
    impl ::std::convert::From<TransferFromCall> for LandNFTCalls {
        fn from(var: TransferFromCall) -> Self {
            LandNFTCalls::TransferFrom(var)
        }
    }
    impl ::std::convert::From<TransferOwnershipCall> for LandNFTCalls {
        fn from(var: TransferOwnershipCall) -> Self {
            LandNFTCalls::TransferOwnership(var)
        }
    }
    impl ::std::convert::From<UnpauseCall> for LandNFTCalls {
        fn from(var: UnpauseCall) -> Self {
            LandNFTCalls::Unpause(var)
        }
    }
    #[doc = "Container type for all return fields from the `MAX_TILE_NUM` function with signature `MAX_TILE_NUM()` and selector `[245, 247, 88, 27]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct MaxTileNumReturn(pub ethers::core::types::U256);
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
    #[doc = "Container type for all return fields from the `admin` function with signature `admin()` and selector `[248, 81, 164, 64]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct AdminReturn(pub ethers::core::types::Address);
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
    #[doc = "Container type for all return fields from the `baseURI` function with signature `baseURI()` and selector `[108, 3, 96, 235]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct BaseURIReturn(pub String);
    #[doc = "Container type for all return fields from the `commissionRate` function with signature `commissionRate()` and selector `[94, 161, 214, 248]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct CommissionRateReturn(pub ethers::core::types::U256);
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
    #[doc = "Container type for all return fields from the `firstOwners` function with signature `firstOwners(uint256)` and selector `[239, 175, 240, 60]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct FirstOwnersReturn(pub ethers::core::types::Address);
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
    #[doc = "Container type for all return fields from the `getTokenPrice` function with signature `getTokenPrice(uint256)` and selector `[196, 87, 251, 55]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct GetTokenPriceReturn(pub ethers::core::types::U256);
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
    #[doc = "Container type for all return fields from the `isOwned` function with signature `isOwned(uint256)` and selector `[89, 159, 104, 156]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct IsOwnedReturn(pub bool);
    #[doc = "Container type for all return fields from the `landBank` function with signature `landBank()` and selector `[239, 211, 41, 115]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct LandBankReturn(pub ethers::core::types::Address);
    #[doc = "Container type for all return fields from the `merkleRoot` function with signature `merkleRoot()` and selector `[46, 180, 167, 171]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct MerkleRootReturn(pub [u8; 32]);
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
    #[doc = "Container type for all return fields from the `nextId` function with signature `nextId()` and selector `[97, 184, 206, 140]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct NextIdReturn(pub ethers::core::types::U256);
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
    #[doc = "Container type for all return fields from the `paused` function with signature `paused()` and selector `[92, 151, 90, 187]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct PausedReturn(pub bool);
    #[doc = "Container type for all return fields from the `price` function with signature `price()` and selector `[160, 53, 177, 254]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct PriceReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `rareClaimedPeriod` function with signature `rareClaimedPeriod()` and selector `[164, 235, 203, 174]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct RareClaimedPeriodReturn(pub ethers::core::types::U256);
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
    #[doc = "Container type for all return fields from the `tilesBought` function with signature `tilesBought()` and selector `[196, 39, 226, 181]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct TilesBoughtReturn(pub ethers::core::types::U256);
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
    #[doc = "Container type for all return fields from the `totalTileNum` function with signature `totalTileNum()` and selector `[97, 139, 42, 221]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct TotalTileNumReturn(pub ethers::core::types::U256);
    #[doc = "`Coordonate(uint256,uint256)`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct Coordonate {
        pub lat: ethers::core::types::U256,
        pub long: ethers::core::types::U256,
    }
    #[doc = "`Pixel((uint256,uint256),(uint256,uint256),(uint256,uint256),(uint256,uint256))`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct Pixel {
        pub a: Coordonate,
        pub b: Coordonate,
        pub c: Coordonate,
        pub d: Coordonate,
    }
}
