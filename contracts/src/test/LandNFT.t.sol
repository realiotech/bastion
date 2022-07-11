// SPDX-License-Identifier: MIT
pragma solidity 0.8.15;

// import "ds-test/test.sol";
import "forge-std/Test.sol";
// import "forge-std/Vm.sol";
import "../LandNFT.sol";
import "../SwapToken.sol";
import {Utilities} from "./utils/Utilities.sol";

contract LandNFTTest is Test {
    // Vm vm = Vm(0x7109709ECfa91a80626fF3989D68f67F5b1DD12D);
    address public admin;
    address payable public devFund =
        payable(0x00EEDFadcC5102B00B1d90D23d135128A29c8B38);
    address public landBank = 0xE1277a3465B92E329c49991D4b95Bc779ba43765;
    address public swapToken;
    address public owner = 0xb4c79daB8f259C7Aee6E5b2Aa729821864227e84;
    address public nftReceiver = 0x892D509964C144f501Aaa8fb1a57069789D65Ce4;
    address public rioWhale = 0x94c3857520E9151b34814FbF8B477368F4a97ea7;
    uint64 public totalTileNum;
    string public baseURI;
    uint256 public nextId;
    uint256 public commissionRate;
    uint256 public constant maxTileNum = 10**10;
    uint256 public price;
    address private constant RIO_TOKEN =
        0xf21661D0D1d76d3ECb8e1B9F1c923DBfffAe4097;
    address private constant WETH = 0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2;

    SwapToken swap;
    LandNFT landNFT;
    Utilities internal utils;

    function setUp() public {
        utils = new Utilities();
        // address payable[] memory users = utils.createUsers(2);
        swap = new SwapToken();
        landNFT = new LandNFT(address(swap), devFund, landBank);
    }

    function testInitialization() public {
        assertEq(landNFT.devFund(), devFund);
        assertEq(landNFT.landBank(), landBank);
        assertEq(landNFT.admin(), owner);
        assertEq(landNFT.commissionRate(), 10);
        assertEq(landNFT.baseURI(), "Realio");
    }

    function testSetAdmin() public {
        address newAdmin = 0x118849c94F887210D933f59Cd962002dF28cB896;
        vm.prank(owner);
        landNFT.setAdmin(newAdmin);
        assertEq(landNFT.admin(), newAdmin);
    }

    function testSetPrice() public {
        uint256 newPrice = 100;
        vm.prank(owner);
        landNFT.setPrice(newPrice);
        assertEq(landNFT.price(), newPrice);
    }

    function testSetCommissionRate() public {
        uint256 newCommissionRate = 20;
        vm.prank(owner);
        landNFT.setCommissionRate(newCommissionRate);
        assertEq(landNFT.commissionRate(), newCommissionRate);
    }

    function testSetLandBank() public {
        address newLandBank = 0x118849c94F887210D933f59Cd962002dF28cB896;
        vm.prank(owner);
        landNFT.setLandBank(payable(newLandBank));
        assertEq(landNFT.landBank(), newLandBank);
    }

    function testPause() public {
        vm.prank(owner);
        landNFT.pause();
        assertEq(landNFT.paused(), true);
    }

    function testUnpause() public {
        vm.prank(owner);
        landNFT.pause();
        assertEq(landNFT.paused(), true);
        vm.prank(owner);
        landNFT.unpause();
        assertEq(landNFT.paused(), false);
    }

    function testSafeMintWithRio() public {
        console.log("Begning ETH balance Test", address(this).balance);
        console.log("Begining ETH balance Dev Fund", address(devFund).balance);
        vm.startPrank(rioWhale);
        uint256[] memory regions = new uint256[](3);
        regions[0] = 1;
        regions[1] = 2;
        regions[2] = 3;
        uint256 rioAmount = 3 * 5 * 10**20;
        IERC20(RIO_TOKEN).approve(address(landNFT), rioAmount);
        // vm.expectEmit(true, true, true, true);
        landNFT.mint(regions, rioAmount);
        vm.stopPrank();
        // Check Whale Balance is reduced by tile price
        // assertEq(
        //     IERC20(RIO_TOKEN).balanceOf(rioWhale),
        //     (IERC20(RIO_TOKEN).balanceOf(rioWhale) - 3 * 5 * 10**20)
        // );
        // Assert that the tile is minted to the rioWhale
        assertEq(3, landNFT.balanceOf(rioWhale));
        assertEq(3, landNFT.tilesBought());
        // Assert that Developer Fund is increased by 20 % of the sale price
        // assertEq(IERC20(WETH).balanceOf(devFund), 300000000000000000000);
        // Assert that the dev fund is increased by 20 % of the sale price.
        address[] memory path = new address[](2);
        path[0] = address(RIO_TOKEN);
        path[1] = address(WETH);
        // FIXME 
        assertEq(
            IERC20(WETH).balanceOf(devFund),
            landNFT.getAmountOutMin(((rioAmount * 20) / 100), path)
        );
        // Assert that the LandBank is increased by 80 % of the sale price
        assertEq(1200000000000000000000, IERC20(RIO_TOKEN).balanceOf(landBank));
    }

    // function testSafeMintWithETH() public {
    //     uint256 id = 1;
    //     uint256 price = 100;
    //     uint256 commissionRate = 20;
    // }

    // function testGetLength() public {
    //     uint256 length = landNFT.getLength();
    //     assertEq(length, 0);
    // }

    // function testGetEthPrice() public {
    //     uint256 ethPrice = landNFT.getEthPrice();
    //     assertEq(ethPrice, 0);
    // }

    // function testTokenURI() public {
    //     string memory uri = landNFT.tokenURI(1);
    //     assertEq(uri, "");
    // }
    fallback() external payable {}

    receive() external payable {}
}
