// SPDX-License-Identifier: MIT
pragma solidity 0.8.15;

import "forge-std/Test.sol";
import "../LandNFT.sol";
import "../Marketplace.sol";
import {Utilities} from "./utils/Utilities.t.sol";

contract MarketplaceTest is Test {
    // address public landNFT;
    address public admin;
    address public devFund;
    address public landBank;
    address public brokeDude;
    address public ethDude;
    address public owner;

    LandNFT landNFT;
    Utilities internal utils;
    Marketplace marketplace;

    function setUp() public {
        utils = new Utilities();
        address payable[] memory users = utils.createUsers(5);
        devFund = users[0];
        landBank = users[1];
        brokeDude = users[2];
        ethDude = users[3];
        owner = users[4];

        vm.label(devFund, "devFund");
        vm.label(landBank, "landBank");
        vm.label(brokeDude, "brokeDude");
        vm.label(ethDude, "ethDude");
        vm.label(owner, "owner");
        vm.startPrank(owner);
        landNFT = new LandNFT(devFund, landBank, 0.01 ether);
        marketplace = new Marketplace(address(landNFT));
        vm.stopPrank();
    }

    function test_Initialization() public {
        assertEq(marketplace.landNFT(), address(landNFT));
        assertEq(marketplace.admin(), owner);
        vm.expectRevert(MarketPlaceCannotSetAddressZero.selector);
        marketplace = new Marketplace(address(0));

        // assertEq(marketplace.devFund(), devFund);
        // assertEq(marketplace.landBank(), landBank);
        // assertEq(marketplace.commissionRate(), 10);
        // assertEq(marketplace.baseURI(), "Realio");
        // assertEq(marketplace.price(), 0.01 ether);
    }

    function test_SetLandBank() public {
        address newLandBank = address(0);
        vm.expectRevert(MarketPlaceNotAuthorized.selector);
        marketplace.setLandBank(newLandBank);
        vm.prank(owner);
        vm.expectRevert(MarketPlaceCannotSetAddressZero.selector);
        marketplace.setLandBank(newLandBank);
        newLandBank = 0x118849c94F887210D933f59Cd962002dF28cB896;
        console.log("msg.sender", msg.sender);
        console.log("marketplace admin", marketplace.admin());
        vm.prank(owner);
        marketplace.setLandBank(newLandBank);
        assertEq(marketplace.landBank(), newLandBank);
        vm.stopPrank();
    }

    function test_Sell() public {}

    function test_Buy() public {}

    function test_CreateAuction() public {}

    function test_Bid() public {}

    function test_WithdrawAuction() public {}

    function test_CanFinalizeAuction() public {}

    function test_AuctionFinalise() public {}

    function test_SellToLandBank() public {}

    function test_BuyFromLandBank() public {}

    function test_WithdrawFromLandBank() public {}
}
