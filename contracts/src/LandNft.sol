// SPDX-License-Identifier: MIT
pragma solidity >=0.8.15;

import {IERC20} from "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import {ERC721A} from "ERC721A/contracts/ERC721A.sol";
import {Strings} from "@openzeppelin/contracts/utils/Strings.sol";
import {Ownable} from "@openzeppelin/contracts/access/Ownable.sol";
import {Pausable} from "@openzeppelin/contracts/security/Pausable.sol";
import {ReentrancyGuard} from "@rari-capital/solmate/src/utils/ReentrancyGuard.sol";
import {SafeTransferLib} from "@rari-capital/solmate/src/utils/SafeTransferLib.sol";

import "./interfaces/IUniswapV2Router.sol";

error CannotSetAddressZero();
error NoTilesSelected();
error RegionAlreadyOwned();
error NotAuthorised();
error ComissionOutOfAllowedRange();
error InsufficientBalance();
error InvalidToken();
error NonExistentTokenURI();
error TransferFailed();
error MaxTilesReached();

/// @title The Realioverse Land NFT
/// @author Samuel Dare (samuel@realio.fund)
/// @notice This contract implements the logic for the Realioverse Land NFT
/// @dev This contract is based on the Realioverse Land NFT contract
contract LandNFT is ERC721A, Ownable, Pausable, ReentrancyGuard {
    using Strings for uint256;

    address swapToken;
    // RIO token address
    address private constant RIO_TOKEN =
        0xf21661D0D1d76d3ECb8e1B9F1c923DBfffAe4097;
    address public constant WETH = 0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2;
    address private constant UNISWAP_V2_ROUTER =
        0x7a250d5630B4cF539739dF2C5dAcb4c659F2488D;
    // admin : can set all parameters
    address public admin;
    // store all funds by ETH
    address public devFund;
    // store some fund by EIO
    address public landBank;
    string public baseURI;
    uint256 public nextId;
    uint256 public commissionRate;
    uint256 public constant MAX_TILE_NUM = 10**10; // we have to set the correct max tile number
    uint256 public price; // each tile costs 500 RIO
    uint256 public tilesBought; // total supply of tiles

    mapping(uint256 => bool) public isOwned;
    // mapping(uint256 => uint256[]) public regionNumbers;
    mapping(uint256 => address) public firstOwners;

    event AdminChanged(address indexed newAdmin, address indexed oldAdmin);
    event DevFundChanged(
        address indexed newDevFund,
        address indexed oldDevFund
    );
    event CommissionRateChanged(
        uint256 indexed newCommission,
        uint256 indexed oldCommission
    );
    event LandBankChanged(
        address indexed newLandBank,
        address indexed oldLandBank
    );
    event ContractPaused(bool indexed paused);
    event ContractUnpaused(bool indexed paused);
    event LandSold(address indexed buyer, uint256[] indexed region);

    //check if the region belongs to somebody.
    // TODO: Check this doesnt make much sense
    modifier notOwned(uint256[] memory region) {
        if (region.length == 0) {
            revert NoTilesSelected();
        }
        bool ownerStatus;
        for (uint256 i = 0; i < region.length; i++) {
            if (isOwned[region[i]]) {
                ownerStatus = true;
                break;
            }
        }
        if (ownerStatus) {
            revert RegionAlreadyOwned();
        }
        _;
    }

    constructor(
        address _swapLibAddr,
        address _devFund,
        address _landBank
    ) ERC721A("RealioVerse", "RVRS") {
        if (
            _swapLibAddr == address(0) ||
            _devFund == address(0) ||
            _landBank == address(0)
        ) {
            revert CannotSetAddressZero();
        }
        // TODO: Update these for prod. If these contracts are not deployed before this ,
        // it might make sense not to add them to the constructor.
        devFund = _devFund;
        // to prevent error we have to set the landbank address to msg.sender
        landBank = _landBank;
        admin = msg.sender;
        commissionRate = 10;
        price = 5 * 10**20;
        swapToken = _swapLibAddr;
        // we don't need this
        baseURI = "Realio";
    }

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
    /// @param region The tiles selected
    /// @param rioAmount The amount of RIO to be transferred
    function mint(
        uint256[] memory region,
        // address token,
        uint256 rioAmount
    ) external payable notOwned(region) whenNotPaused {
        if (totalSupply() >= MAX_TILE_NUM) {
            revert MaxTilesReached();
        }
        if (rioAmount > 0) {
            if (rioAmount < price * region.length) {
                revert InsufficientBalance();
            }

            // number of tiles to be mints
            uint256 numberOfTiles = region.length;
            // Loop through the number of tiles and mark them as owned
            for (uint256 i; i < numberOfTiles; i++) {
                isOwned[region[i]] = true;
            }
            // Approve the amount of RIO to be transferred
            IERC20(RIO_TOKEN).approve(address(this), rioAmount);
            // Transfer the amount of RIO to the contract
            bool success = IERC20(RIO_TOKEN).transferFrom(
                msg.sender,
                address(this),
                rioAmount
            );

            if (!success) {
                revert TransferFailed();
            }

            uint256 amountIn = (rioAmount * 20) / 100;
            // Approve the Uniswap Router contract
            IERC20(RIO_TOKEN).approve(UNISWAP_V2_ROUTER, amountIn);

            address[] memory path = new address[](2);
            path[0] = address(RIO_TOKEN);
            path[1] = address(WETH);
            uint256 amountOutMin = getAmountOutMin(amountIn, path);
            IUniswapV2Router(UNISWAP_V2_ROUTER).swapExactTokensForETH(
                amountIn,
                amountOutMin,
                path,
                devFund,
                block.timestamp
            );
            success = IERC20(RIO_TOKEN).transfer(
                landBank,
                (rioAmount * 80) / 100
            );

            if (!success) {
                revert TransferFailed();
            }

            _safeMint(msg.sender, numberOfTiles);

            tilesBought += numberOfTiles;
            emit LandSold(msg.sender, region);
        }
    }

    function getAmountOutMin(uint256 _amountIn, address[] memory path)
        public
        view
        returns (uint256)
    {
        uint256[] memory amountOutMins = IUniswapV2Router(UNISWAP_V2_ROUTER)
            .getAmountsOut(_amountIn, path);
        return amountOutMins[path.length - 1];
    }

    /// View Functions

    /// @notice Returns the number of tiles selects
    /// @param index the index of the tile selected
    // function getLength(uint256 index) external view returns (uint256 len) {
    //     len = regionNumbers[index].length;
    // }

    // /// @notice Returns the price the tile in ETH
    // /// @param _price the price of the token in RIO
    // function getETHPrice(uint256 _price) external view returns (uint256) {
    //     return ISwapToken(swapToken).getAmountOutMin(RIO_TOKEN, WETH, _price);
    // }

    /// @notice Returns a token URI
    /// @param tokenId the id of the token
    function tokenURI(uint256 tokenId)
        public
        view
        virtual
        override
        returns (string memory)
    {
        if (ownerOf(tokenId) == address(0)) {
            revert NonExistentTokenURI();
        }
        return
            bytes(baseURI).length > 0
                ? string(abi.encodePacked(baseURI, tokenId.toString()))
                : "";
    }

    //this function will return the minimum amount from a swap
    //input the 3 parameters below and it will return the minimum amount out
    //this is needed for the swap function above

    receive() external payable {}
}
