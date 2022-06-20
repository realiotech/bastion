// SPDX-License-Identifier: MIT
pragma solidity >=0.8.10;

//import the ERC20 interface

import {IERC20} from "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import "./interfaces/ISwapToken.sol";
import "./interfaces/IUniswapV2Router.sol";
import "./interfaces/IUniswapV2Pair.sol";
import "./interfaces/IUniswapV2Factory.sol";

/// @title The Realioverse Swap Contract
/// @author Samuel Dare (samuel@realio.fund)
/// @notice This contract implements the logic for swapping ETH for Realio tokens.
/// @dev This implement UniswapV2 swap functionality
contract SwapToken {
    //address of the uniswap v2 router
    address private constant UNISWAP_V2_ROUTER =
        0x7a250d5630B4cF539739dF2C5dAcb4c659F2488D;

    //WETH Address
    address private constant WETH = 0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2;

    //this swap function is used to trade from one token to another
    //the inputs are self explainatory
    //token in = the token address you want to trade out of
    //token out = the token address you want as the output of this trade
    //amount in = the amount of tokens you are sending in
    //amount out Min = the minimum amount of tokens you want out of the trade
    //to = the address you want the tokens to be sent to

    function swap(
        address _tokenIn,
        address _tokenOut,
        uint256 _amountIn,
        uint256 _amountOutMin,
        address _to
    ) external payable {
        require(
            (_tokenIn == WETH || _tokenOut == WETH),
            "Must one of tokens is WETH"
        );

        //path is an array of addresses.
        //this path array will have 3 addresses [tokenIn, WETH, tokenOut]
        //the if statement below takes into account if token in or token out is WETH.  then the path is only 2 addresses
        address[] memory path;

        path = new address[](2);
        path[0] = _tokenIn;
        path[1] = _tokenOut;
        if (_tokenIn == WETH) {
            IUniswapV2Router(UNISWAP_V2_ROUTER).swapExactETHForTokens{
                value: msg.value
            }(_amountOutMin, path, _to, block.timestamp);
        } else if (_tokenOut == WETH) {
            //first we need to transfer the amount in tokens from the msg.sender to this contract
            //this contract will have the amount of in tokens
            IERC20(_tokenIn).transferFrom(msg.sender, address(this), _amountIn);
            //next we need to allow the uniswapv2 router to spend the token we just sent to this contract
            //by calling IERC20 approve you allow the uniswap contract to spend the tokens in this contract
            IERC20(_tokenIn).approve(UNISWAP_V2_ROUTER, _amountIn);

            IUniswapV2Router(UNISWAP_V2_ROUTER).swapExactTokensForETH(
                _amountIn,
                _amountOutMin,
                path,
                _to,
                block.timestamp
            );
        }
    }

    //this function will return the minimum amount from a swap
    //input the 3 parameters below and it will return the minimum amount out
    //this is needed for the swap function above
    function getAmountOutMin(
        address _tokenIn,
        address _tokenOut,
        uint256 _amountIn
    ) external view returns (uint256) {
        //path is an array of addresses.
        //this path array will have 3 addresses [tokenIn, WETH, tokenOut]
        //the if statement below takes into account if token in or token out is WETH.  then the path is only 2 addresses
        address[] memory path;
        if (_tokenIn == WETH || _tokenOut == WETH) {
            path = new address[](2);
            path[0] = _tokenIn;
            path[1] = _tokenOut;
        } else {
            path = new address[](3);
            path[0] = _tokenIn;
            path[1] = WETH;
            path[2] = _tokenOut;
        }

        uint256[] memory amountOutMins = IUniswapV2Router(UNISWAP_V2_ROUTER)
            .getAmountsOut(_amountIn, path);
        return amountOutMins[path.length - 1];
    }
}
