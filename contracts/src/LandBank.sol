// SPDX-License-Identifier: MIT
pragma solidity >=0.8.15;

import {IERC20} from "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import {ReentrancyGuard} from "@rari-capital/solmate/src/utils/ReentrancyGuard.sol";
import {ILandNFT} from "./interfaces/ILandNft.sol";

import "./interfaces/ILandBank.sol";
import "./interfaces/IUniswapV2Router.sol";
import "./interfaces/IUniswapV2Pair.sol";
import "./errors.sol";

contract LandBank is ReentrancyGuard {
    // RIO token address
    address private constant RIO_TOKEN =
        0xf21661D0D1d76d3ECb8e1B9F1c923DBfffAe4097;
    address public constant WETH = 0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2;

    address public owner;
    address public landNft;
    address public devFund;
    address private constant UNISWAP_V2_ROUTER =
        0x7a250d5630B4cF539739dF2C5dAcb4c659F2488D;

    mapping(uint256 => uint256) timelapse;

    event LandSold(
        address _seller,
        uint256[] landId,
        uint256 amount,
        uint256 at
    );

    event LandBought(
        address _buyer,
        uint256[] landId,
        uint256 amount,
        uint256 at
    );

    constructor(address _devFund, address _landNft) {
        require(
            _devFund != address(0) && _landNft != address(0),
            "can't set zero address"
        );
        devFund = _devFund;
        landNft = _landNft;
    }

    /**
    @notice msg.sender can buy land directly from LandBank contract
    @param _tokenIds array of land user would like to buy
    todo getPrice for landNFT allow , how the price is determined 
    */
    function buyLandFromBank(uint256[] memory _tokenIds)
        external
        payable
        nonReentrant
    {
        uint256 numberOfPx = _tokenIds.length;
        uint256 amountToSend = numberOfPx * getPrice();

        for (uint256 i; i < _tokenIds.length; i++) {
            if (timelapse[_tokenIds[i]] + 5 days > block.timestamp) {
                revert coolDown();
            }
        }
        // Approve the Uniswap Router contract
        address[] memory path = new address[](2);
        path[0] = address(RIO_TOKEN);
        path[1] = address(WETH);
        if (msg.value == 0) {
            if (IERC20(RIO_TOKEN).balanceOf(msg.sender) < amountToSend) {
                revert InsufficientRio();
            }
            // Transfer the amount of RIO to the contract
            bool success = IERC20(RIO_TOKEN).transferFrom(
                msg.sender,
                address(this),
                amountToSend
            );
            if (!success) {
                revert FailedTransfer();
            }
            uint256 amountIn = (amountToSend * 10) / 100;
            uint256 amountOutMin = getAmountOutMin(amountIn, path);
            IERC20(RIO_TOKEN).approve(UNISWAP_V2_ROUTER, amountIn);
            IUniswapV2Router(UNISWAP_V2_ROUTER).swapExactTokensForETH(
                amountIn,
                amountOutMin,
                path,
                devFund,
                block.timestamp
            );
        } else if (
            msg.value > 0 || IERC20(RIO_TOKEN).balanceOf(msg.sender) == 0
        ) {
            uint256 minAmount = getAmountOutMin(amountToSend, path);
            if (msg.value < minAmount) {
                revert InsufficientEthBalance();
            }
            // 10% of funds are sent to the dev address
            payable(devFund).transfer(msg.value / 10);
            // swap rest of funds 90 % to RIO token
            uint256 amountIn = (msg.value * 9) / 10;
            address[] memory new_path = new address[](2);
            new_path[0] = address(WETH);
            new_path[1] = address(RIO_TOKEN);
            uint256 amountOutMin = getAmountOutMin(amountIn, path);
            IUniswapV2Router(UNISWAP_V2_ROUTER).swapExactETHForTokens{
                value: amountIn
            }(amountOutMin, new_path, address(this), block.timestamp);
        }

        for (uint256 j; j < numberOfPx; j++) {
            ILandNFT(landNft).safeTransferFrom(
                address(this),
                msg.sender,
                _tokenIds[j]
            );
            timelapse[_tokenIds[j]] = block.timestamp;
        }

        emit LandBought(msg.sender, _tokenIds, amountToSend, block.timestamp);
    }

    /**
    @notice msg.sender can buy land directly from LandBank contract
    @param _tokenIds array of land user would like to buy
    todo getPrice for landNFT allow , how the price is determined 
    */
    function sellLandToBank(uint256[] memory _tokenIds) external nonReentrant {
        uint256 numberOfPx = _tokenIds.length;
        uint256 amountToSend;

        unchecked {
            amountToSend = getPrice() * numberOfPx;
        }

        for (uint256 i; i < numberOfPx; i++) {
            ILandNFT(landNft).transferFrom(
                msg.sender,
                address(this),
                _tokenIds[i]
            );
            timelapse[_tokenIds[i]] = block.timestamp;
        }
        IERC20(RIO_TOKEN).transfer(msg.sender, amountToSend);
        emit LandSold(msg.sender, _tokenIds, amountToSend, block.timestamp);
    }

    function getAmountOutMin(uint256 _amountIn, address[] memory path)
        public
        view
        returns (uint256)
    {
        uint256[] memory amountOutMins = IUniswapV2Router(UNISWAP_V2_ROUTER)
            .getAmountsOut(_amountIn, path);
        return amountOutMins[path.length - 1];
    }

    /**
     * getPrice function determines the price landBank value for pixel
     * price = total LandBank Holding / Number of pixel circulating
     */
    function getPrice() public returns (uint256) {
        uint256 holding = IERC20(RIO_TOKEN).balanceOf(address(this));
        uint256 PIXEL_SUPPLY = ILandNFT(landNft).totalTileNum();
        uint256 landPrice;
        unchecked {
            landPrice = ((holding / PIXEL_SUPPLY) * 12) / 10;
        }
        return landPrice;
    }

    receive() external payable {}
}
