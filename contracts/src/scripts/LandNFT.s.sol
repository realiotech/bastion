// SPDX-License-Identifier: MIT
pragma solidity >=0.8.15;

import "../LandNFT.sol";
import "../LandBank.sol";

import "forge-std/Script.sol";

contract DeploymentLandNFT is Script {
    address public constant dev = 0x27a1876A09581E02E583E002E42EC1322abE9655;
    uint256 public price = 0.01 ether;

    function run() external {
        vm.startBroadcast();
        LandNFT nft = new LandNFT(dev, price);
        LandBank bank = new LandBank(dev, address(nft));
        nft.setLandBank(payable(address(bank)));
    uint256 public price = 20e18;

    function run() external {
        vm.startBroadcast();
        new LandNFT(dev, dev, price);
        vm.stopBroadcast();
    }
}
