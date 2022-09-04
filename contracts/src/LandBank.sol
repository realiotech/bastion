// SPDX-License-Identifier: MIT
pragma solidity >=0.8.15;

import {IERC20} from "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import {ReentrancyGuard} from "@rari-capital/solmate/src/utils/ReentrancyGuard.sol";
import {ILandNFT} from "./interfaces/ILandNft.sol";

import "./interfaces/ILandBank.sol";
import "./interfaces/IUniswapV2Router.sol";

error InsufficientRio();
error InvalidLand();
error coolDown();
error FailedTransfer();

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

    uint256 public landPrice;

    mapping(uint256 => uint256) timelapse;

    event LandSold(
        address _seller,
        uint256[] landId,
        uint256 amount,
        uint256 at
    );

    constructor(address _marketplace, address _landNft) {
        require(
            _marketplace != address(0) && _landNft != address(0),
            "can't set zero address"
        );
        owner = _marketplace;
        landNft = _landNft;
    }

    function setPrice(uint256 _price) external {
        if (msg.sender != owner) {
            revert();
        }
        landPrice = _price;
    }

    function buyLandFromBank(uint256[] memory _tokenIds) external nonReentrant {
        uint256 i;
        uint256 numberOfPx = _tokenIds.length;
        uint256 amountToSend = numberOfPx * landPrice;
        address[] memory path = new address[](2);
        path[0] = address(RIO_TOKEN);
        path[1] = address(WETH);

        // Approve the Uniswap Router contract

        for (i; i < _tokenIds.length; i++) {
            if (timelapse[_tokenIds[i]] + 5 days > block.timestamp) {
                revert coolDown();
            }
        }
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
        uint256 amountIn = (amountToSend * 20) / 100;
        uint256 amountOutMin = getAmountOutMin(amountIn, path);
        IERC20(RIO_TOKEN).approve(UNISWAP_V2_ROUTER, amountIn);
        IUniswapV2Router(UNISWAP_V2_ROUTER).swapExactTokensForETH(
            amountIn,
            amountOutMin,
            path,
            owner,
            block.timestamp
        );
        for (i; i < numberOfPx; i++) {
            ILandNFT(landNft).transferFrom(
                address(this),
                msg.sender,
                _tokenIds[i]
            );
        }
    }

    function sellLandToBank(uint256[] memory _tokenIds) external nonReentrant {
        uint256 numberOfPx = _tokenIds.length;
        uint256 amountToSend;
        uint256 i;
        unchecked {
            amountToSend =
                (IERC20(RIO_TOKEN).balanceOf(address(this)) /
                    ILandNFT(landNft).totalTileNum()) *
                numberOfPx;
        }

        for (i; i < numberOfPx; i++) {
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

    // function withdraw(address _beneficiary, uint256 _amount) external {
    //     require(
    //         owner == msg.sender,
    //         "Only owner contract can call withdraw function"
    //     );
    //     require(
    //         _amount <= IERC20(RIO_TOKEN).balanceOf(address(this)),
    //         "Too large amount"
    //     );
    //     IERC20(RIO_TOKEN).transfer(_beneficiary, _amount);
    // }
    function getAmountOutMin(uint256 _amountIn, address[] memory path)
        public
        view
        returns (uint256)
    {
        uint256[] memory amountOutMins = IUniswapV2Router(UNISWAP_V2_ROUTER)
            .getAmountsOut(_amountIn, path);
        return amountOutMins[path.length - 1];
    }
}
