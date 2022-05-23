// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

interface ISwapToken {
  function swap(address _tokenIn, address _tokenOut, uint256 _amountIn, uint256 _amountOutMin, address _to) external payable;
  function getAmountOutMin(address _tokenIn, address _tokenOut, uint256 _amountIn) external view returns (uint256);
}