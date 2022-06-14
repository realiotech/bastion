// SPDX-License-Identifier: MIT
pragma solidity >=0.8.10;

import {IERC20} from "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import {ERC721} from "@rari-capital/solmate/src/tokens/ERC721.sol";
import {Strings} from "@openzeppelin/contracts/utils/Strings.sol";
import {Ownable} from "@openzeppelin/contracts/access/Ownable.sol";
import {Pausable} from "@openzeppelin/contracts/security/Pausable.sol";
import {ReentrancyGuard} from "@rari-capital/solmate/src/utils/ReentrancyGuard.sol";
import {SafeTransferLib} from "@rari-capital/solmate/src/utils/SafeTransferLib.sol";
import {ISwapToken} from "./interfaces/ISwapToken.sol";

error CannotSetAddressZero();
error NoTilesSelected();
error RegionAlreadyOwned();
error NotAuthorised();
error ComissionOutOfAllowedRange();
error InsufficientBalance();
error InvalidToken();

/// @title The Realioverse Land NFT
/// @author Samuel Dare (samuel@realio.fund)
/// @notice This contract implements the logic for the Realioverse Land NFT
/// @dev This contract is based on the Realioverse Land NFT contract
contract LandNFT is ERC721, Ownable, Pausable, ReentrancyGuard {
    using Strings for uint256;

    address swapToken;
    // RIO token address
    address private constant RIO_TOKEN =
        0xf21661D0D1d76d3ECb8e1B9F1c923DBfffAe4097;
    address public constant WETH = 0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2;
    // admin : can set all parameters
    address public admin;
    // store all funds by ETH
    address public devFund;
    // store some fund by EIO
    address public landBank;
    uint64 public totalTileNum;
    string public baseURI;
    uint256 public nextId;
    uint256 public commissionRate;
    uint256 public constant maxTileNum = 10**10; // we have to set the correct max tile number
    uint256 public price; // each tile costs 500 RIO

    mapping(uint256 => bool) public isOwned;
    mapping(uint256 => uint256[]) public regionNumbers;
    mapping(uint256 => address) public firstOwners;

    event LandNFTCreated(address landNft, address landBank, address swapToken);
    event LandNFTDestroyed(address landNft);
    event LandNFTTileBought(address buyer, uint256 tokenId);
    event LandNFTTileSold(address seller, uint256 tokenId);
    event LandNFTTileWithdrawn(address beneficiary, uint256 amount);
    event LandNFTTileDeposited(address beneficiary, uint256 amount);
    event AdminChanged(address newAdmin, address oldAdmin);
    event DevFundChanged(address newDevFund, address oldDevFund);
    event CommissionRateChanged(uint256 newCommission, uint256 oldCommission);
    event LandBankChanged(address newLandBank, address oldLandBank);
    event ContractPaused(bool paused);
    event ContractUnpaused(bool paused);

    //check if the region belongs to somebody.
    modifier notOwned(uint256[] memory region) {
        if (region.length == 0) {
            revert NoTilesSelected();
        }
        bool ownerStatus;
        for (uint256 i = 0; i < region.length; i++) {
            if (isOwned[region[i]] == true) {
                ownerStatus = true;
                break;
            }
        }
        if (!ownerStatus) {
            revert RegionAlreadyOwned();
        }
        _;
    }

    constructor(address _swapLibAddr) ERC721("RealioVerse", "RVRS") {
        require(_swapLibAddr != address(0), "can't set zero address");
        devFund = msg.sender;
        // to prevent error we have to set the landbank address to msg.sender
        landBank = msg.sender;
        admin = msg.sender;
        commissionRate = 10;
        price = 5 * 10**20;
        swapToken = _swapLibAddr;
        // we don't need this
        baseURI = "Realio";
    }

    // we need this function when only test
    // function initializeSwapParams(address _rioToken, address _router) public {
    //     require(msg.sender == admin, "Only admin can initialize");
    //     RIO_TOKEN = _rioToken;
    //     router = _router; // set uniswap router
    //     // pool = _pool; // set uniswap RIO/ETH pool
    // }

    /// State Changing Functions

    /// @notice Changes the admin address
    /// @param _newAdmin The new admin address
    function setAdmin(address _newAdmin) external {
        if (msg.sender != admin) {
            revert NotAuthorised();
        } else if (_newAdmin == address(0)) {
            revert CannotSetAddressZero();
        } else {
            admin = _newAdmin;
        }
        emit AdminChanged(admin, msg.sender);
    }

    /// @notice Changes the price of a tile
    /// @param _price The new admin address
    function setPrice(uint256 _price) external {
        if (msg.sender != admin) {
            revert NotAuthorised();
        } else {
            price = _price;
        }
    }

    /// @notice Changes the commission rate
    /// @param _commissionRate The new commission rate
    function setCommissionRate(uint256 _commissionRate) external {
        uint256 oldCommissionRate = commissionRate;
        if (msg.sender != admin) {
            revert NotAuthorised();
        } else if (_commissionRate > 20 || _commissionRate < 0) {
            revert ComissionOutOfAllowedRange();
        } else {
            commissionRate = _commissionRate;
        }
        emit CommissionRateChanged(commissionRate, oldCommissionRate);
    }

    /// @notice Changes the devfund address
    /// @param _devFund The new commission rate
    function setDevFund(address payable _devFund) external {
        address oldDevFund = devFund;
        if (msg.sender != admin) {
            revert NotAuthorised();
        } else if (_devFund == address(0)) {
            revert CannotSetAddressZero();
        } else {
            devFund = _devFund;
        }
        emit DevFundChanged(devFund, oldDevFund);
    }

    /// @notice Changes the landbak address
    /// @param _landBank The new landbank address
    function setLandBank(address payable _landBank) external {
        address oldLandBank = landBank;
        if (msg.sender != admin) {
            revert NotAuthorised();
        } else if (_landBank == address(0)) {
            revert CannotSetAddressZero();
        } else {
            landBank = _landBank;
        }
        emit LandBankChanged(landBank, oldLandBank);
    }

    /// @notice Pauses the contract
    function pause() external whenNotPaused {
        if (msg.sender != admin) {
            revert NotAuthorised();
        } else {
            _pause();
        }

        emit ContractPaused(true);
    }

    /// @notice Unpauses the contract
    function unpause() external whenPaused {
        if (msg.sender != admin) {
            revert NotAuthorised();
        } else {
            _unpause();
        }
        emit ContractUnpaused(true);
    }

    /// @notice Mints a new NFT according the the tiles selected
    /// @param to The beneficiary address
    /// @param region The tiles selected
    /// @param rioAmount The amount of RIO to be transferred
    // TODO: add guard to prevent
    function safeMint(
        address to,
        uint256[] memory region,
        address token,
        uint256 rioAmount
    ) external payable notOwned(region) whenNotPaused {
        // uint256 gas = gasleft();
        if (token == RIO_TOKEN) {
            if (rioAmount < price * region.length) {
                revert InsufficientBalance();
            } else {
                IERC20(RIO_TOKEN).transferFrom(
                    msg.sender,
                    address(this),
                    rioAmount
                );
                IERC20(RIO_TOKEN).approve(swapToken, (rioAmount * 20) / 100);
                uint256 minAmount = ISwapToken(swapToken).getAmountOutMin(
                    RIO_TOKEN,
                    WETH,
                    (rioAmount * 20) / 100
                );
                ISwapToken(swapToken).swap(
                    RIO_TOKEN,
                    WETH,
                    (rioAmount * 20) / 100,
                    minAmount,
                    devFund
                );
                IERC20(RIO_TOKEN).transfer(landBank, (rioAmount * 80) / 100);
            }
        } else {
            uint256 minAmount = ISwapToken(swapToken).getAmountOutMin(
                WETH,
                RIO_TOKEN,
                msg.value
            );
            require(minAmount >= price * region.length, "low value");
            // require(false, "here");
            ISwapToken(swapToken).swap{value: (msg.value * 80) / 100}(
                WETH,
                RIO_TOKEN,
                (rioAmount * 80) / 100,
                (minAmount * 80) / 100,
                landBank
            );
            (bool success, ) = devFund.call{value: (msg.value * 20) / 100}("");
            require(success, "Transfer to devFund failed.");
        }
        _mint(to, nextId);
        firstOwners[nextId] = to;
        regionNumbers[nextId] = region;
        nextId++;
        totalTileNum += uint64(region.length);
        require(totalTileNum <= maxTileNum, "Max limit of tile");
        //set Ownership
        for (uint256 i = 0; i < region.length; i++) {
            isOwned[region[i]] = true;
        }
    }

    /// View Functions

    function getLength(uint256 index) external view returns (uint256 len) {
        len = regionNumbers[index].length;
    }

    function getETHPrice(uint256 _price) external view returns (uint256) {
        return ISwapToken(swapToken).getAmountOutMin(RIO_TOKEN, WETH, _price);
    }

    function tokenURI(uint256 tokenId)
        public
        view
        override
        returns (string memory)
    {
        // require(ownerOf[tokenId] == address(0), "Token does not exist");
        return
            bytes(baseURI).length > 0
                ? string(abi.encodePacked(baseURI, tokenId.toString(), ".json"))
                : "";
    }
}
