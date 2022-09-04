// SPDX-License-Identifier: MIT
pragma solidity 0.8.15;

import "forge-std/Test.sol";
import "../LandBank.sol";
import "../landNFT.sol";
import {Utilities} from "./utils/Utilities.t.sol";
import {ILandNFT} from "../interfaces/ILandNft.sol";

contract LandBankTest is Test {
    address public admin;
    address public devFund;
    address public brokeDude;
    address public ethDude;
    address public swapToken;
    address public owner = 0xb4c79daB8f259C7Aee6E5b2Aa729821864227e84;
    address public nftReceiver = 0x892D509964C144f501Aaa8fb1a57069789D65Ce4;
    address public rioWhale = 0x94c3857520E9151b34814FbF8B477368F4a97ea7;
    uint64 public totalTileNum;
    string public baseURI;
    uint256 public nextId;
    uint256 public commissionRate;
    uint256 public constant maxTileNum = 10**10;

    address private constant RIO_TOKEN =
        0xf21661D0D1d76d3ECb8e1B9F1c923DBfffAe4097;
    address private constant WETH = 0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2;
    uint256 public constant FEE_MULTIPLIER = (97 / uint256(100));
    uint256 public price = 0.01 ether;

    LandNFT landNFT;
    LandBank landBank;
    Utilities internal utils;

    function setUp() public {
        utils = new Utilities();
        address payable[] memory users = utils.createUsers(4);
        admin = users[1];
        devFund = users[2];
        landNFT = new LandNFT(devFund, price);
        // landNFT = new LandNFT(devFund, price);
        landBank = new LandBank(admin, address(landNFT));
        landNFT.setLandBank(payable(address(landBank)));
    }

    function test_initialization() public {
        assertEq(landBank.landNft(), address(landNFT));
    }

    function mint_utils() public returns (uint256 rioAmount) {
        vm.startPrank(rioWhale);
        uint256[] memory regions = new uint256[](3);
        regions[0] = 1;
        regions[1] = 2;
        regions[2] = 3;
        address[] memory path = new address[](2);
        path[0] = address(WETH);
        path[1] = address(RIO_TOKEN);
        rioAmount = landNFT.getTokenPrice(price * regions.length);
        IERC20(RIO_TOKEN).approve(address(landNFT), rioAmount);
        landNFT.mint(regions, rioAmount);
    }

    function test_mint_funds() public {
        assertEq(IERC20(RIO_TOKEN).balanceOf(address(landBank)), 0);
        uint256 oldDevBalance = address(devFund).balance;
        uint256 rioAmount = mint_utils();
        // check if landBank receive 80% of the rio amount
        assertEq(
            IERC20(RIO_TOKEN).balanceOf(address(landBank)),
            (rioAmount * 80) / 100
        );
        // check if dev fund balance increase after the mint
        assert(address(devFund).balance > oldDevBalance);
    }

    function test_sell_to_land() public {
        mint_utils();
        // vm.startPrank(rioWhale);
        ILandNFT(address(landNFT)).approve(address(landBank), 0);
        uint256[] memory tokenId = new uint256[](1);
        tokenId[0] = 0;
        landBank.sellLandToBank(tokenId);
        assertEq(
            ILandNFT(address(landNFT)).ownerOf(tokenId[0]),
            address(landBank)
        );
    }
}
