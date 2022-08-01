// SPDX-License-Identifier: MIT
pragma solidity 0.8.15;

import "forge-std/Test.sol";
import "../LandNFT.sol";
import "../Marketplace.sol";
import {Utilities} from "./utils/Utilities.t.sol";

contract MarketplaceTest is Test {
    address public admin;
    address public devFund;
    address public landBank;
    address public brokeDude;
    address public ethDude;

    LandNFT landNFT;
    Utilities internal utils;
    Marketplace marketplace;

    function setUp() public {
        utils = new Utilities();
        address payable[] memory users = utils.createUsers(4);
        devFund = users[0];
        landBank = users[1];
        brokeDude = users[2];
        ethDude = users[3];
        vm.label(devFund, "devFund");
        vm.label(landBank, "landBank");
        vm.label(brokeDude, "brokeDude");
        vm.label(ethDude, "ethDude");
        landNFT = new LandNFT(devFund, landBank, 0.01 ether);
        marketplace = new Marketplace(address(landNFT), landBank);
    }

    function testSetLandBank() public {
        address newLandBank = 0x118849c94F887210D933f59Cd962002dF28cB896;
        vm.prank(owner);
        marketplace.setLandBank(newLandBank);
        assertEq(marketplace.landBank(), newLandBank);
    }

    function testInitialization() public {}

    function testSell() public {}

    function testBuy() public {}

    function testCreateAuction() public {}

    function testBid() public {}

    function testWithdrawAuction() public {}

    function testCanFinalizeAuction() public {}

    function testAuctionFinalise() public {}

    function testSellToLandBank() public {}

    function testBuyFromLandBank() public {}

    function testWithdrawFromLandBank() public {}
}
