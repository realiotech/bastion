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
    pub static LANDNFT_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            ethers :: core :: utils :: __serde_json :: from_str ("[{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_devFund\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"_price\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"constructor\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"ApprovalCallerNotOwnerNorApproved\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"ApprovalQueryForNonexistentToken\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"ApproveToCaller\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"BalanceQueryForZeroAddress\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"CannotSetAddressZero\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"ComissionOutOfAllowedRange\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"InsufficientBalance\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"MaxTilesReached\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"MintERC2309QuantityExceedsLimit\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"MintToZeroAddress\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"MintZeroQuantity\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"NonExistentTokenURI\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"NotAuthorised\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"OwnerQueryForNonexistentToken\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"OwnershipNotInitializedForExtraData\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"TransferCallerNotOwnerNorApproved\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"TransferFailed\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"TransferFromIncorrectOwner\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"TransferToNonERC721ReceiverImplementer\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"TransferToZeroAddress\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"URIQueryForNonexistentToken\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"newAdmin\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"oldAdmin\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"AdminChanged\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"owner\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"approved\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"tokenId\",\"type\":\"uint256\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"Approval\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"owner\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"operator\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"bool\",\"name\":\"approved\",\"type\":\"bool\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"ApprovalForAll\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"newCommission\",\"type\":\"uint256\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"oldCommission\",\"type\":\"uint256\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"CommissionRateChanged\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"fromTokenId\",\"type\":\"uint256\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"toTokenId\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"address\",\"name\":\"from\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"to\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"ConsecutiveTransfer\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"bool\",\"name\":\"paused\",\"type\":\"bool\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"ContractPaused\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"bool\",\"name\":\"paused\",\"type\":\"bool\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"ContractUnpaused\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"newDevFund\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"oldDevFund\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"DevFundChanged\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"newLandBank\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"oldLandBank\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"LandBankChanged\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"buyer\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"struct ILandNFT.Pixel[]\",\"name\":\"region\",\"type\":\"tuple[]\",\"components\":[{\"internalType\":\"struct ILandNFT.Coordonate\",\"name\":\"a\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"uint256\",\"name\":\"lat\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"long\",\"type\":\"uint256\",\"components\":[]}]},{\"internalType\":\"struct ILandNFT.Coordonate\",\"name\":\"b\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"uint256\",\"name\":\"lat\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"long\",\"type\":\"uint256\",\"components\":[]}]},{\"internalType\":\"struct ILandNFT.Coordonate\",\"name\":\"c\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"uint256\",\"name\":\"lat\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"long\",\"type\":\"uint256\",\"components\":[]}]},{\"internalType\":\"struct ILandNFT.Coordonate\",\"name\":\"d\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"uint256\",\"name\":\"lat\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"long\",\"type\":\"uint256\",\"components\":[]}]}],\"indexed\":true}],\"type\":\"event\",\"name\":\"LandSold\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"previousOwner\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"newOwner\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"OwnershipTransferred\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"account\",\"type\":\"address\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"Paused\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"from\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"to\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"tokenId\",\"type\":\"uint256\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"Transfer\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"account\",\"type\":\"address\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"Unpaused\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"MAX_TILE_NUM\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"WETH\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"admin\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"to\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"tokenId\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"approve\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"owner\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"balanceOf\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"baseURI\",\"outputs\":[{\"internalType\":\"string\",\"name\":\"\",\"type\":\"string\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"commissionRate\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"devFund\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"firstOwners\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_amountIn\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"address[]\",\"name\":\"path\",\"type\":\"address[]\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getAmountOutMin\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"tokenId\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getApproved\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getTokenPrice\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"owner\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"operator\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"isApprovedForAll\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"isOwned\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"landBank\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"struct ILandNFT.Pixel[]\",\"name\":\"region\",\"type\":\"tuple[]\",\"components\":[{\"internalType\":\"struct ILandNFT.Coordonate\",\"name\":\"a\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"uint256\",\"name\":\"lat\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"long\",\"type\":\"uint256\",\"components\":[]}]},{\"internalType\":\"struct ILandNFT.Coordonate\",\"name\":\"b\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"uint256\",\"name\":\"lat\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"long\",\"type\":\"uint256\",\"components\":[]}]},{\"internalType\":\"struct ILandNFT.Coordonate\",\"name\":\"c\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"uint256\",\"name\":\"lat\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"long\",\"type\":\"uint256\",\"components\":[]}]},{\"internalType\":\"struct ILandNFT.Coordonate\",\"name\":\"d\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"uint256\",\"name\":\"lat\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"long\",\"type\":\"uint256\",\"components\":[]}]}]},{\"internalType\":\"uint256\",\"name\":\"rioAmount\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"payable\",\"type\":\"function\",\"name\":\"mint\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"name\",\"outputs\":[{\"internalType\":\"string\",\"name\":\"\",\"type\":\"string\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"nextId\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"owner\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"tokenId\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"ownerOf\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"pause\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"paused\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"price\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"renounceOwnership\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"from\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"to\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"tokenId\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"safeTransferFrom\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"from\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"to\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"tokenId\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"_data\",\"type\":\"bytes\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"safeTransferFrom\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_newAdmin\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setAdmin\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"operator\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bool\",\"name\":\"approved\",\"type\":\"bool\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setApprovalForAll\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_commissionRate\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setCommissionRate\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address payable\",\"name\":\"_devFund\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setDevFund\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address payable\",\"name\":\"_landBank\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setLandBank\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_price\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setPrice\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"bytes4\",\"name\":\"interfaceId\",\"type\":\"bytes4\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"supportsInterface\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"symbol\",\"outputs\":[{\"internalType\":\"string\",\"name\":\"\",\"type\":\"string\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"tilesBought\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"tokenId\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"tokenURI\",\"outputs\":[{\"internalType\":\"string\",\"name\":\"\",\"type\":\"string\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"totalSupply\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"totalTileNum\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"from\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"to\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"tokenId\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"transferFrom\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"newOwner\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"transferOwnership\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"unpause\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"payable\",\"type\":\"receive\",\"outputs\":[]}]") . expect ("invalid abi")
        });
    #[doc = r" Bytecode of the #name contract"]
    pub static LANDNFT_BYTECODE: ethers::contract::Lazy<ethers::core::types::Bytes> =
        ethers::contract::Lazy::new(|| {
            "0x608060405260016009553480156200001657600080fd5b5060405162002ddb38038062002ddb833981016040819052620000399162000199565b6040518060400160405280600b81526020016a5265616c696f566572736560a81b815250604051806040016040528060048152602001635256525360e01b81525081600290816200008b91906200027a565b5060036200009a82826200027a565b50506000805550620000ac3362000147565b6008805460ff60a01b191690556001600160a01b038216620000e1576040516397b43c7960e01b815260040160405180910390fd5b600b80546001600160a01b0384166001600160a01b031991821617909155600a805490911633178155600f5560108190556040805180820190915260068152655265616c696f60d01b6020820152600d906200013e90826200027a565b50505062000346565b600880546001600160a01b038381166001600160a01b0319831681179093556040519116919082907f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e090600090a35050565b60008060408385031215620001ad57600080fd5b82516001600160a01b0381168114620001c557600080fd5b6020939093015192949293505050565b634e487b7160e01b600052604160045260246000fd5b600181811c908216806200020057607f821691505b6020821081036200022157634e487b7160e01b600052602260045260246000fd5b50919050565b601f8211156200027557600081815260208120601f850160051c81016020861015620002505750805b601f850160051c820191505b8181101562000271578281556001016200025c565b5050505b505050565b81516001600160401b03811115620002965762000296620001d5565b620002ae81620002a78454620001eb565b8462000227565b602080601f831160018114620002e65760008415620002cd5750858301515b600019600386901b1c1916600185901b17855562000271565b600085815260208120601f198616915b828110156200031757888601518255948401946001909101908401620002f6565b5085821015620003365787850151600019600388901b60f8161c191681555b5050505050600190811b01905550565b612a8580620003566000396000f3fe60806040526004361061024a5760003560e01c806370a0823111610139578063b88d4fde116100b6578063efaff03c1161007a578063efaff03c146106b1578063efd32973146106e7578063f0640d9514610707578063f2fde38b14610727578063f5f7581b14610747578063f851a4401461076057600080fd5b8063b88d4fde146105f2578063c427e2b514610612578063c457fb3714610628578063c87b56dd14610648578063e985e9c51461066857600080fd5b806395d89b41116100fd57806395d89b411461055f578063a035b1fe14610574578063a22cb4651461058a578063ad5c4648146105aa578063ae4db919146105d257600080fd5b806370a08231146104d7578063715018a6146104f75780638456cb591461050c5780638da5cb5b1461052157806391b7f5ed1461053f57600080fd5b80634390d2a8116101c7578063618b2add1161018b578063618b2add1461045757806361b8ce8c1461046c5780636352211e146104825780636c0360eb146104a2578063704b6c02146104b757600080fd5b80634390d2a8146103b257806348334429146103d2578063599f689c146103f25780635c975abb146104225780635ea1d6f81461044157600080fd5b806318160ddd1161020e57806318160ddd1461031a57806319fac8fd1461033d57806323b872dd1461035d5780633f4ba83a1461037d57806342842e0e1461039257600080fd5b8063019ae5991461025657806301ffc9a71461026b57806306fdde03146102a0578063081812fc146102c2578063095ea7b3146102fa57600080fd5b3661025157005b600080fd5b61026961026436600461217f565b610780565b005b34801561027757600080fd5b5061028b61028636600461228e565b610f60565b60405190151581526020015b60405180910390f35b3480156102ac57600080fd5b506102b5610fb2565b6040516102979190612303565b3480156102ce57600080fd5b506102e26102dd366004612316565b611044565b6040516001600160a01b039091168152602001610297565b34801561030657600080fd5b50610269610315366004612344565b611088565b34801561032657600080fd5b50600154600054035b604051908152602001610297565b34801561034957600080fd5b50610269610358366004612316565b611128565b34801561036957600080fd5b50610269610378366004612370565b6111ae565b34801561038957600080fd5b50610269611346565b34801561039e57600080fd5b506102696103ad366004612370565b6113af565b3480156103be57600080fd5b50600b546102e2906001600160a01b031681565b3480156103de57600080fd5b5061032f6103ed3660046123b1565b6113ca565b3480156103fe57600080fd5b5061028b61040d366004612316565b60136020526000908152604090205460ff1681565b34801561042e57600080fd5b50600854600160a01b900460ff1661028b565b34801561044d57600080fd5b5061032f600f5481565b34801561046357600080fd5b5060115461032f565b34801561047857600080fd5b5061032f600e5481565b34801561048e57600080fd5b506102e261049d366004612316565b61147f565b3480156104ae57600080fd5b506102b561148a565b3480156104c357600080fd5b506102696104d2366004612457565b611518565b3480156104e357600080fd5b5061032f6104f2366004612457565b6115b7565b34801561050357600080fd5b50610269611606565b34801561051857600080fd5b5061026961161a565b34801561052d57600080fd5b506008546001600160a01b03166102e2565b34801561054b57600080fd5b5061026961055a366004612316565b611683565b34801561056b57600080fd5b506102b56116b7565b34801561058057600080fd5b5061032f60105481565b34801561059657600080fd5b506102696105a5366004612482565b6116c6565b3480156105b657600080fd5b506102e273c02aaa39b223fe8d0a0e5c4f27ead9083c756cc281565b3480156105de57600080fd5b506102696105ed366004612457565b61175b565b3480156105fe57600080fd5b5061026961060d3660046124bb565b611804565b34801561061e57600080fd5b5061032f60115481565b34801561063457600080fd5b5061032f610643366004612316565b61184e565b34801561065457600080fd5b506102b5610663366004612316565b6118fc565b34801561067457600080fd5b5061028b61068336600461257f565b6001600160a01b03918216600090815260076020908152604080832093909416825291909152205460ff1690565b3480156106bd57600080fd5b506102e26106cc366004612316565b6014602052600090815260409020546001600160a01b031681565b3480156106f357600080fd5b50600c546102e2906001600160a01b031681565b34801561071357600080fd5b50610269610722366004612457565b61198c565b34801561073357600080fd5b50610269610742366004612457565b611a35565b34801561075357600080fd5b5061032f6402540be40081565b34801561076c57600080fd5b50600a546102e2906001600160a01b031681565b610788611ab0565b6402540be40061079b6001546000540390565b106107b95760405163e74fd80b60e01b815260040160405180910390fd5b8151811515806107c7575034155b15610cf65760408051600280825260608201835260009260208301908036833701905050905073f21661d0d1d76d3ecb8e1b9f1c923dbfffae409781600081518110610815576108156125ad565b60200260200101906001600160a01b031690816001600160a01b03168152505073c02aaa39b223fe8d0a0e5c4f27ead9083c756cc28160018151811061085d5761085d6125ad565b60200260200101906001600160a01b031690816001600160a01b03168152505060006108908360105461064391906125d9565b9050808410156108b357604051631e9acf1760e31b815260040160405180910390fd5b60405163095ea7b360e01b81523060048201526024810185905273f21661d0d1d76d3ecb8e1b9f1c923dbfffae40979063095ea7b3906044016020604051808303816000875af115801561090b573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019061092f91906125f8565b506040516323b872dd60e01b81523360048201523060248201526044810185905260009073f21661d0d1d76d3ecb8e1b9f1c923dbfffae4097906323b872dd906064016020604051808303816000875af1158015610991573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906109b591906125f8565b9050806109d5576040516312171d8360e31b815260040160405180910390fd5b600060646109e48760146125d9565b6109ee919061262b565b60405163095ea7b360e01b8152737a250d5630b4cf539739df2c5dacb4c659f2488d60048201526024810182905290915073f21661d0d1d76d3ecb8e1b9f1c923dbfffae40979063095ea7b3906044016020604051808303816000875af1158015610a5d573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190610a8191906125f8565b50604080516002808252606082018352909160208301908036833701905050935073f21661d0d1d76d3ecb8e1b9f1c923dbfffae409784600081518110610aca57610aca6125ad565b60200260200101906001600160a01b031690816001600160a01b03168152505073c02aaa39b223fe8d0a0e5c4f27ead9083c756cc284600181518110610b1257610b126125ad565b60200260200101906001600160a01b031690816001600160a01b0316815250506000610b3e82866113ca565b600b546040516318cbafe560e01b8152919250737a250d5630b4cf539739df2c5dacb4c659f2488d916318cbafe591610b8b91869186918b916001600160a01b0316904290600401612683565b600060405180830381600087803b158015610ba557600080fd5b505af1158015610bb9573d6000803e3d6000fd5b5050600c5473f21661d0d1d76d3ecb8e1b9f1c923dbfffae4097925063a9059cbb91506001600160a01b03166064610bf28b60506125d9565b610bfc919061262b565b6040516001600160e01b031960e085901b1681526001600160a01b03909216600483015260248201526044016020604051808303816000875af1158015610c47573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190610c6b91906125f8565b925082610c8b576040516312171d8360e31b815260040160405180910390fd5b610c953387611afd565b8560116000828254610ca791906126bf565b9091555050604051610cba9089906126d7565b6040519081900381209033907ffedaf99886e624862075ed4c59d51e3ec75e512a2ddd085a09a1234f2a486bc790600090a35050505050505050565b6000341180610d03575081155b15610f5b578251601054610d1791906125d9565b341015610d3757604051631e9acf1760e31b815260040160405180910390fd5b600b546001600160a01b03166108fc6064610d533460146125d9565b610d5d919061262b565b6040518115909202916000818181858888f19350505050158015610d85573d6000803e3d6000fd5b5060006064610d953460506125d9565b610d9f919061262b565b604080516002808252606082018352929350600092909160208301908036833701905050905073c02aaa39b223fe8d0a0e5c4f27ead9083c756cc281600081518110610ded57610ded6125ad565b60200260200101906001600160a01b031690816001600160a01b03168152505073f21661d0d1d76d3ecb8e1b9f1c923dbfffae409781600181518110610e3557610e356125ad565b60200260200101906001600160a01b031690816001600160a01b0316815250506000610e6183836113ca565b600c54604051637ff36ab560e01b8152919250737a250d5630b4cf539739df2c5dacb4c659f2488d91637ff36ab5918691610eae91869188916001600160a01b031690429060040161276f565b60006040518083038185885af1158015610ecc573d6000803e3d6000fd5b50505050506040513d6000823e601f3d908101601f19168201604052610ef591908101906127a4565b50610f003385611afd565b8360116000828254610f1291906126bf565b9091555050604051610f259087906126d7565b6040519081900381209033907ffedaf99886e624862075ed4c59d51e3ec75e512a2ddd085a09a1234f2a486bc790600090a35050505b505050565b60006301ffc9a760e01b6001600160e01b031983161480610f9157506380ac58cd60e01b6001600160e01b03198316145b80610fac5750635b5e139f60e01b6001600160e01b03198316145b92915050565b606060028054610fc190612835565b80601f0160208091040260200160405190810160405280929190818152602001828054610fed90612835565b801561103a5780601f1061100f5761010080835404028352916020019161103a565b820191906000526020600020905b81548152906001019060200180831161101d57829003601f168201915b5050505050905090565b600061104f82611b1b565b61106c576040516333d1c03960e21b815260040160405180910390fd5b506000908152600660205260409020546001600160a01b031690565b60006110938261147f565b9050336001600160a01b038216146110cc576110af8133610683565b6110cc576040516367d9dca160e11b815260040160405180910390fd5b60008281526006602052604080822080546001600160a01b0319166001600160a01b0387811691821790925591518593918516917f8c5be1e5ebec7d5bd14f71427d1e84f3dd0314c0f7b2291e5b200ac8c7c3b92591a4505050565b600f54600a546001600160a01b0316331461115657604051631648fd0160e01b815260040160405180910390fd5b601482111561117857604051637d1dd3df60e11b815260040160405180910390fd5b600f829055604051819083907f74b81a9e0217358c4b0755d3032738dc303e980dde2980905160b1d8e7b68ba690600090a35050565b60006111b982611b42565b9050836001600160a01b0316816001600160a01b0316146111ec5760405162a1148160e81b815260040160405180910390fd5b60008281526006602052604090208054338082146001600160a01b038816909114176112395761121c8633610683565b61123957604051632ce44b5f60e11b815260040160405180910390fd5b6001600160a01b03851661126057604051633a954ecd60e21b815260040160405180910390fd5b801561126b57600082555b6001600160a01b038681166000908152600560205260408082208054600019019055918716808252919020805460010190554260a01b17600160e11b17600085815260046020526040812091909155600160e11b841690036112fd576001840160008181526004602052604081205490036112fb5760005481146112fb5760008181526004602052604090208490555b505b83856001600160a01b0316876001600160a01b03167fddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef60405160405180910390a4505050505050565b61134e611bb0565b600a546001600160a01b0316331461137957604051631648fd0160e01b815260040160405180910390fd5b611381611c00565b6040516001907fc6cd34d367248623c114617f3cf4e7d54b15f11b158367408ee3b4c0ff1a5e2e90600090a2565b610f5b83838360405180602001604052806000815250611804565b60405163d06ca61f60e01b81526000908190737a250d5630b4cf539739df2c5dacb4c659f2488d9063d06ca61f90611408908790879060040161286f565b600060405180830381865afa158015611425573d6000803e3d6000fd5b505050506040513d6000823e601f3d908101601f1916820160405261144d91908101906127a4565b9050806001845161145e9190612888565b8151811061146e5761146e6125ad565b602002602001015191505092915050565b6000610fac82611b42565b600d805461149790612835565b80601f01602080910402602001604051908101604052809291908181526020018280546114c390612835565b80156115105780601f106114e557610100808354040283529160200191611510565b820191906000526020600020905b8154815290600101906020018083116114f357829003601f168201915b505050505081565b600a546001600160a01b0316331461154357604051631648fd0160e01b815260040160405180910390fd5b6001600160a01b03811661156a576040516397b43c7960e01b815260040160405180910390fd5b600a80546001600160a01b0319166001600160a01b0383169081179091556040513391907f7e644d79422f17c01e4894b5f4f588d331ebfa28653d42ae832dc59e38c9798f90600090a350565b60006001600160a01b0382166115e0576040516323d3ad8160e21b815260040160405180910390fd5b506001600160a01b031660009081526005602052604090205467ffffffffffffffff1690565b61160e611c55565b6116186000611caf565b565b611622611ab0565b600a546001600160a01b0316331461164d57604051631648fd0160e01b815260040160405180910390fd5b611655611d01565b6040516001907f752d7e161ff5146f80e3820893176eb40532811e5e20400dfdde57455213706a90600090a2565b600a546001600160a01b031633146116ae57604051631648fd0160e01b815260040160405180910390fd5b60108190555b50565b606060038054610fc190612835565b336001600160a01b038316036116ef5760405163b06307db60e01b815260040160405180910390fd5b3360008181526007602090815260408083206001600160a01b03871680855290835292819020805460ff191686151590811790915590519081529192917f17307eab39ab6107e8899845ad3d59bd9653f200f220920489ca2b5937696c31910160405180910390a35050565b600b54600a546001600160a01b039182169116331461178d57604051631648fd0160e01b815260040160405180910390fd5b6001600160a01b0382166117b4576040516397b43c7960e01b815260040160405180910390fd5b600b80546001600160a01b0319166001600160a01b03848116918217909255604051918316917ff87e12ba363db684b1b69a530d850a8a3f416932cd031e008ef71e42a1d8845090600090a35050565b61180f8484846111ae565b6001600160a01b0383163b156118485761182b84848484611d44565b611848576040516368d2bf6b60e11b815260040160405180910390fd5b50505050565b600080730b85b3000bef3e26e01428d1b525a532ea7513b89050600080826001600160a01b0316630902f1ac6040518163ffffffff1660e01b8152600401606060405180830381865afa1580156118a9573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906118cd91906128bb565b506001600160701b039182169350169050816118e982876125d9565b6118f3919061262b565b95945050505050565b606060006119098361147f565b6001600160a01b0316036119305760405163d872946b60e01b815260040160405180910390fd5b6000600d805461193f90612835565b90501161195b5760405180602001604052806000815250610fac565b600d61196683611e30565b604051602001611977929190612927565b60405160208183030381529060405292915050565b600c54600a546001600160a01b03918216911633146119be57604051631648fd0160e01b815260040160405180910390fd5b6001600160a01b0382166119e5576040516397b43c7960e01b815260040160405180910390fd5b600c80546001600160a01b0319166001600160a01b03848116918217909255604051918316917f04c90a5bd107b5b753ce9758599ca56cffaadede0b5f6c4b3a375a5effe208d490600090a35050565b611a3d611c55565b6001600160a01b038116611aa75760405162461bcd60e51b815260206004820152602660248201527f4f776e61626c653a206e6577206f776e657220697320746865207a65726f206160448201526564647265737360d01b60648201526084015b60405180910390fd5b6116b481611caf565b600854600160a01b900460ff16156116185760405162461bcd60e51b815260206004820152601060248201526f14185d5cd8589b194e881c185d5cd95960821b6044820152606401611a9e565b611b17828260405180602001604052806000815250611f31565b5050565b6000805482108015610fac575050600090815260046020526040902054600160e01b161590565b600081600054811015611b975760008181526004602052604081205490600160e01b82169003611b95575b80600003611b8e575060001901600081815260046020526040902054611b6d565b9392505050565b505b604051636f96cda160e11b815260040160405180910390fd5b600854600160a01b900460ff166116185760405162461bcd60e51b815260206004820152601460248201527314185d5cd8589b194e881b9bdd081c185d5cd95960621b6044820152606401611a9e565b611c08611bb0565b6008805460ff60a01b191690557f5db9ee0a495bf2e6ff9c91a7834c1ba4fdd244a5e8aa4e537bd38aeae4b073aa335b6040516001600160a01b03909116815260200160405180910390a1565b6008546001600160a01b031633146116185760405162461bcd60e51b815260206004820181905260248201527f4f776e61626c653a2063616c6c6572206973206e6f7420746865206f776e65726044820152606401611a9e565b600880546001600160a01b038381166001600160a01b0319831681179093556040519116919082907f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e090600090a35050565b611d09611ab0565b6008805460ff60a01b1916600160a01b1790557f62e78cea01bee320cd4e420270b5ea74000d11b0c9f74754ebdbfc544b05a258611c383390565b604051630a85bd0160e11b81526000906001600160a01b0385169063150b7a0290611d799033908990889088906004016129c8565b6020604051808303816000875af1925050508015611db4575060408051601f3d908101601f19168201909252611db191810190612a05565b60015b611e12573d808015611de2576040519150601f19603f3d011682016040523d82523d6000602084013e611de7565b606091505b508051600003611e0a576040516368d2bf6b60e11b815260040160405180910390fd5b805181602001fd5b6001600160e01b031916630a85bd0160e11b1490505b949350505050565b606081600003611e575750506040805180820190915260018152600360fc1b602082015290565b8160005b8115611e815780611e6b81612a22565b9150611e7a9050600a8361262b565b9150611e5b565b60008167ffffffffffffffff811115611e9c57611e9c61209c565b6040519080825280601f01601f191660200182016040528015611ec6576020820181803683370190505b5090505b8415611e2857611edb600183612888565b9150611ee8600a86612a3b565b611ef39060306126bf565b60f81b818381518110611f0857611f086125ad565b60200101906001600160f81b031916908160001a905350611f2a600a8661262b565b9450611eca565b611f3b8383611f9e565b6001600160a01b0383163b15610f5b576000548281035b611f656000868380600101945086611d44565b611f82576040516368d2bf6b60e11b815260040160405180910390fd5b818110611f52578160005414611f9757600080fd5b5050505050565b6000805490829003611fc35760405163b562e8dd60e01b815260040160405180910390fd5b6001600160a01b03831660008181526005602090815260408083208054680100000000000000018802019055848352600490915281206001851460e11b4260a01b178317905582840190839083907fddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef8180a4600183015b81811461207257808360007fddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef600080a460010161203a565b508160000361209357604051622e076360e81b815260040160405180910390fd5b60005550505050565b634e487b7160e01b600052604160045260246000fd5b6040516080810167ffffffffffffffff811182821017156120d5576120d561209c565b60405290565b604051601f8201601f1916810167ffffffffffffffff811182821017156121045761210461209c565b604052919050565b600067ffffffffffffffff8211156121265761212661209c565b5060051b60200190565b60006040828403121561214257600080fd5b6040516040810181811067ffffffffffffffff821117156121655761216561209c565b604052823581526020928301359281019290925250919050565b600080604080848603121561219357600080fd5b833567ffffffffffffffff8111156121aa57600080fd5b8401601f810186136121bb57600080fd5b803560206121d06121cb8361210c565b6120db565b82815260089290921b830181019181810190898411156121ef57600080fd5b938201935b8385101561226857610100858b03121561220e5760008081fd5b6122166120b2565b6122208b87612130565b815261222e8b888801612130565b8482015261223f8b60808801612130565b878201526122508b60c08801612130565b606082015282526101009490940193908201906121f4565b9997909101359750505050505050565b6001600160e01b0319811681146116b457600080fd5b6000602082840312156122a057600080fd5b8135611b8e81612278565b60005b838110156122c65781810151838201526020016122ae565b838111156118485750506000910152565b600081518084526122ef8160208601602086016122ab565b601f01601f19169290920160200192915050565b602081526000611b8e60208301846122d7565b60006020828403121561232857600080fd5b5035919050565b6001600160a01b03811681146116b457600080fd5b6000806040838503121561235757600080fd5b82356123628161232f565b946020939093013593505050565b60008060006060848603121561238557600080fd5b83356123908161232f565b925060208401356123a08161232f565b929592945050506040919091013590565b600080604083850312156123c457600080fd5b8235915060208084013567ffffffffffffffff8111156123e357600080fd5b8401601f810186136123f457600080fd5b80356124026121cb8261210c565b81815260059190911b8201830190838101908883111561242157600080fd5b928401925b828410156124485783356124398161232f565b82529284019290840190612426565b80955050505050509250929050565b60006020828403121561246957600080fd5b8135611b8e8161232f565b80151581146116b457600080fd5b6000806040838503121561249557600080fd5b82356124a08161232f565b915060208301356124b081612474565b809150509250929050565b600080600080608085870312156124d157600080fd5b84356124dc8161232f565b93506020858101356124ed8161232f565b935060408601359250606086013567ffffffffffffffff8082111561251157600080fd5b818801915088601f83011261252557600080fd5b8135818111156125375761253761209c565b612549601f8201601f191685016120db565b9150808252898482850101111561255f57600080fd5b808484018584013760008482840101525080935050505092959194509250565b6000806040838503121561259257600080fd5b823561259d8161232f565b915060208301356124b08161232f565b634e487b7160e01b600052603260045260246000fd5b634e487b7160e01b600052601160045260246000fd5b60008160001904831182151516156125f3576125f36125c3565b500290565b60006020828403121561260a57600080fd5b8151611b8e81612474565b634e487b7160e01b600052601260045260246000fd5b60008261263a5761263a612615565b500490565b600081518084526020808501945080840160005b838110156126785781516001600160a01b031687529582019590820190600101612653565b509495945050505050565b85815284602082015260a0604082015260006126a260a083018661263f565b6001600160a01b0394909416606083015250608001529392505050565b600082198211156126d2576126d26125c3565b500190565b815160009082906020808601845b838110156127635781518051805187526020908101518188015284820151805160408901520151606087015261274d612734608088016040840151805182526020908101519082015260400190565b6060830151805182526020908101519082015260400190565b50506101009490940193908201906001016126e5565b50929695505050505050565b848152608060208201526000612788608083018661263f565b6001600160a01b03949094166040830152506060015292915050565b600060208083850312156127b757600080fd5b825167ffffffffffffffff8111156127ce57600080fd5b8301601f810185136127df57600080fd5b80516127ed6121cb8261210c565b81815260059190911b8201830190838101908783111561280c57600080fd5b928401925b8284101561282a57835182529284019290840190612811565b979650505050505050565b600181811c9082168061284957607f821691505b60208210810361286957634e487b7160e01b600052602260045260246000fd5b50919050565b828152604060208201526000611e28604083018461263f565b60008282101561289a5761289a6125c3565b500390565b80516001600160701b03811681146128b657600080fd5b919050565b6000806000606084860312156128d057600080fd5b6128d98461289f565b92506128e76020850161289f565b9150604084015163ffffffff8116811461290057600080fd5b809150509250925092565b6000815161291d8185602086016122ab565b9290920192915050565b600080845481600182811c91508083168061294357607f831692505b6020808410820361296257634e487b7160e01b86526022600452602486fd5b818015612976576001811461298b576129b8565b60ff19861689528415158502890196506129b8565b60008b81526020902060005b868110156129b05781548b820152908501908301612997565b505084890196505b5050505050506118f3818561290b565b6001600160a01b03858116825284166020820152604081018390526080606082018190526000906129fb908301846122d7565b9695505050505050565b600060208284031215612a1757600080fd5b8151611b8e81612278565b600060018201612a3457612a346125c3565b5060010190565b600082612a4a57612a4a612615565b50069056fea2646970667358221220409c09a99b179170f27859e7d9331f4e0a949d551101277fb027324bfa56acf064736f6c634300080f0033" . parse () . expect ("invalid bytecode")
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
    impl<M: ethers::providers::Middleware> std::fmt::Debug for LandNFT<M> {
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
        #[doc = "Calls the contract's `setPrice` (0x91b7f5ed) function"]
        pub fn set_price(
            &self,
            price: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([145, 183, 245, 237], price)
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
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum LandNFTErrors {
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
    }
    impl ethers::core::abi::AbiDecode for LandNFTErrors {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::std::result::Result<Self, ethers::core::abi::AbiError> {
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
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for LandNFTErrors {
        fn encode(self) -> Vec<u8> {
            match self {
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
            }
        }
    }
    impl ::std::fmt::Display for LandNFTErrors {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
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
            }
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
    pub struct SafeTransferFromWithDataCall {
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
        CommissionRate(CommissionRateCall),
        DevFund(DevFundCall),
        FirstOwners(FirstOwnersCall),
        GetAmountOutMin(GetAmountOutMinCall),
        GetApproved(GetApprovedCall),
        GetTokenPrice(GetTokenPriceCall),
        IsApprovedForAll(IsApprovedForAllCall),
        IsOwned(IsOwnedCall),
        LandBank(LandBankCall),
        Mint(MintCall),
        Name(NameCall),
        NextId(NextIdCall),
        Owner(OwnerCall),
        OwnerOf(OwnerOfCall),
        Pause(PauseCall),
        Paused(PausedCall),
        Price(PriceCall),
        RenounceOwnership(RenounceOwnershipCall),
        SafeTransferFrom(SafeTransferFromCall),
        SafeTransferFromWithData(SafeTransferFromWithDataCall),
        SetAdmin(SetAdminCall),
        SetApprovalForAll(SetApprovalForAllCall),
        SetCommissionRate(SetCommissionRateCall),
        SetDevFund(SetDevFundCall),
        SetLandBank(SetLandBankCall),
        SetPrice(SetPriceCall),
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
                <SafeTransferFromWithDataCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(LandNFTCalls::SafeTransferFromWithData(decoded));
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
                <SetPriceCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(LandNFTCalls::SetPrice(decoded));
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
                LandNFTCalls::CommissionRate(element) => element.encode(),
                LandNFTCalls::DevFund(element) => element.encode(),
                LandNFTCalls::FirstOwners(element) => element.encode(),
                LandNFTCalls::GetAmountOutMin(element) => element.encode(),
                LandNFTCalls::GetApproved(element) => element.encode(),
                LandNFTCalls::GetTokenPrice(element) => element.encode(),
                LandNFTCalls::IsApprovedForAll(element) => element.encode(),
                LandNFTCalls::IsOwned(element) => element.encode(),
                LandNFTCalls::LandBank(element) => element.encode(),
                LandNFTCalls::Mint(element) => element.encode(),
                LandNFTCalls::Name(element) => element.encode(),
                LandNFTCalls::NextId(element) => element.encode(),
                LandNFTCalls::Owner(element) => element.encode(),
                LandNFTCalls::OwnerOf(element) => element.encode(),
                LandNFTCalls::Pause(element) => element.encode(),
                LandNFTCalls::Paused(element) => element.encode(),
                LandNFTCalls::Price(element) => element.encode(),
                LandNFTCalls::RenounceOwnership(element) => element.encode(),
                LandNFTCalls::SafeTransferFrom(element) => element.encode(),
                LandNFTCalls::SafeTransferFromWithData(element) => element.encode(),
                LandNFTCalls::SetAdmin(element) => element.encode(),
                LandNFTCalls::SetApprovalForAll(element) => element.encode(),
                LandNFTCalls::SetCommissionRate(element) => element.encode(),
                LandNFTCalls::SetDevFund(element) => element.encode(),
                LandNFTCalls::SetLandBank(element) => element.encode(),
                LandNFTCalls::SetPrice(element) => element.encode(),
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
                LandNFTCalls::CommissionRate(element) => element.fmt(f),
                LandNFTCalls::DevFund(element) => element.fmt(f),
                LandNFTCalls::FirstOwners(element) => element.fmt(f),
                LandNFTCalls::GetAmountOutMin(element) => element.fmt(f),
                LandNFTCalls::GetApproved(element) => element.fmt(f),
                LandNFTCalls::GetTokenPrice(element) => element.fmt(f),
                LandNFTCalls::IsApprovedForAll(element) => element.fmt(f),
                LandNFTCalls::IsOwned(element) => element.fmt(f),
                LandNFTCalls::LandBank(element) => element.fmt(f),
                LandNFTCalls::Mint(element) => element.fmt(f),
                LandNFTCalls::Name(element) => element.fmt(f),
                LandNFTCalls::NextId(element) => element.fmt(f),
                LandNFTCalls::Owner(element) => element.fmt(f),
                LandNFTCalls::OwnerOf(element) => element.fmt(f),
                LandNFTCalls::Pause(element) => element.fmt(f),
                LandNFTCalls::Paused(element) => element.fmt(f),
                LandNFTCalls::Price(element) => element.fmt(f),
                LandNFTCalls::RenounceOwnership(element) => element.fmt(f),
                LandNFTCalls::SafeTransferFrom(element) => element.fmt(f),
                LandNFTCalls::SafeTransferFromWithData(element) => element.fmt(f),
                LandNFTCalls::SetAdmin(element) => element.fmt(f),
                LandNFTCalls::SetApprovalForAll(element) => element.fmt(f),
                LandNFTCalls::SetCommissionRate(element) => element.fmt(f),
                LandNFTCalls::SetDevFund(element) => element.fmt(f),
                LandNFTCalls::SetLandBank(element) => element.fmt(f),
                LandNFTCalls::SetPrice(element) => element.fmt(f),
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
    impl ::std::convert::From<SafeTransferFromWithDataCall> for LandNFTCalls {
        fn from(var: SafeTransferFromWithDataCall) -> Self {
            LandNFTCalls::SafeTransferFromWithData(var)
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
    impl ::std::convert::From<SetPriceCall> for LandNFTCalls {
        fn from(var: SetPriceCall) -> Self {
            LandNFTCalls::SetPrice(var)
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
