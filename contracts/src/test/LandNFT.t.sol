// SPDX-License-Identifier: MIT
pragma solidity 0.8.10;

// import "ds-test/test.sol";
import "forge-std/Test.sol";
// import "forge-std/Vm.sol";
import "../LandNFT.sol";
import "../SwapToken.sol";

import "forge-std/console2.sol";

contract LandNFTTest is Test {
    // Vm vm = Vm(0x7109709ECfa91a80626fF3989D68f67F5b1DD12D);
    address public admin;
    address public devFund;
    address public landBank;
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
    SwapToken swap;
    LandNFT landNFT;

    function setUp() public {
        swap = new SwapToken();
        landNFT = new LandNFT(address(swap));
    }

    function testInitialization() public {
        assertEq(landNFT.devFund(), owner);
        assertEq(landNFT.landBank(), owner);
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
        vm.prank(owner);
        uint256[] memory regions = new uint256[](3);
        regions[0] = 1;
        regions[1] = 2;
        regions[2] = 3;
        uint256 rioAmount = 5 * 10**20;
        landNFT.safeMint(rioWhale, regions, rioAmount);
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
}
