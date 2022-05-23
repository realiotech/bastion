// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import "hardhat/console.sol";

import {IERC20} from "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import {ERC721} from "@rari-capital/solmate/src/tokens/ERC721.sol";
import {Strings} from "@openzeppelin/contracts/utils/Strings.sol";
import {Ownable} from "@openzeppelin/contracts/access/Ownable.sol";
import {Pausable} from "@openzeppelin/contracts/security/Pausable.sol";
import {ReentrancyGuard} from "@rari-capital/solmate/src/utils/ReentrancyGuard.sol";
import {SafeTransferLib} from "@rari-capital/solmate/src/utils/SafeTransferLib.sol";
import {ISwapToken} from "./lib/ISwapToken.sol";

contract LandNFT is ERC721, Ownable, Pausable, ReentrancyGuard {
    using Strings for uint256;

    address swapToken;
    // RIO token address
    address private constant RIO_TOKEN =
        0xf21661D0D1d76d3ECb8e1B9F1c923DBfffAe4097;
    address public constant WETH = 0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2;

    // address public pool;
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

    //check if the region belongs to somebody.
    modifier notOwned(uint256[] memory region) {
        require(region.length >= 1, "Must have at least 1 tile");
        bool ownerStatus;
        for (uint256 i = 0; i < region.length; i++) {
            if (isOwned[region[i]] == true) {
                ownerStatus = true;
                break;
            }
        }
        require(!ownerStatus, "Region is owned already");
        _;
    }

    constructor(address _swapLibAddr) ERC721("RealioVerse", "RVRS") {
        require(_swapLibAddr != address(0), "can't set zero address");
        // baseURI = _baseURI;
        // _setupRole(DEFAULT_ADMIN_ROLE, msg.sender);
        // _setupRole(PAUSER_ROLE, msg.sender);
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

    // we have to call this function when the admin wallet is in danger.
    function setAdmin(address _admin) external {
        require(_admin != address(0), "can't set zero address");
        // require(msg.sender == admin, "Only admin can set admin");
        require(admin == msg.sender, "Only admin can set land unit price");
        admin = _admin;
    }

    function setPrice(uint256 _price) external {
        require(admin == msg.sender, "Only admin can set land unit price");
        price = _price;
    }

    function setCommissionRate(uint256 _commissionRate) external {
        require(admin == msg.sender, "Only admin can set land unit price");
        require(
            _commissionRate <= 20,
            "Commission rate has to be between 0 and 20"
        );
        commissionRate = _commissionRate;
    }

    function setDevFund(address payable _devFund) external {
        require(_devFund != address(0), "can't set zero address");
        require(admin == msg.sender, "Only admin can set land unit price");
        devFund = payable(_devFund);
    }

    function setLandBank(address payable _landBank) external {
        require(_landBank != address(0), "can't set zero address");
        require(admin == msg.sender, "Only admin can set land unit price");
        landBank = payable(_landBank);
    }

    function pause() external whenNotPaused {
        require(admin == msg.sender, "Only admin can set land unit price");
        _pause();
    }

    function unpause() external whenPaused {
        require(admin == msg.sender, "Only admin can set land unit price");
        _unpause();
    }

    function getLength(uint256 index) external view returns (uint256 len) {
        len = regionNumbers[index].length;
    }

    function getETHPrice(uint256 _price) external view returns (uint256) {
        return ISwapToken(swapToken).getAmountOutMin(RIO_TOKEN, WETH, _price);
    }

    // users can mint land NFT using only RIO token
    function safeMint(
        address to,
        uint256[] memory region,
        uint256 rioAmount
    ) external payable notOwned(region) whenNotPaused {
        // uint256 gas = gasleft();
        if (rioAmount > 0) {
            require(rioAmount == price * region.length, "low value");
            // msg.sender must approve this wallet to control amount of RIO token.
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

    function tokenURI(uint256 tokenId)
        public
        view
        override
        returns (string memory)
    {
        require(ownerOf[tokenId] == address(0), "Token does not exist");
        return
            bytes(baseURI).length > 0
                ? string(abi.encodePacked(baseURI, tokenId.toString(), ".json"))
                : "";
    }
}
