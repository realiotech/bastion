// SPDX-License-Identifier: MIT
pragma solidity >=0.8.15;

import {ERC20} from "@openzeppelin/contracts/token/ERC20/ERC20.sol";

contract MockRio is ERC20 {
    constructor() ERC20("MockRio", "MRIO") {}

    function mint(address _account, uint256 _amount) external payable {
        _mint(_account, _amount);
    }
}
