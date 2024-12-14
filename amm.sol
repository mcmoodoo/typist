// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

contract AutomatedMarketMaker {
    address public tokenA;
    address public tokenB;

    uint256 public reserveA;
    uint256 public reserveB;

    uint256 public constant FEE_PERCENT = 30; // 0.3% fee
    uint256 public constant FEE_DENOMINATOR = 10000;

    event LiquidityAdded(address indexed provider, uint256 amountA, uint256 amountB);
    event LiquidityRemoved(address indexed provider, uint256 amountA, uint256 amountB);
    event Swap(address indexed swapper, address tokenIn, uint256 amountIn, address tokenOut, uint256 amountOut);

    constructor(address _tokenA, address _tokenB) {
        require(_tokenA != _tokenB, "Tokens must be different");
        tokenA = _tokenA;
        tokenB = _tokenB;
    }

    function addLiquidity(uint256 amountA, uint256 amountB) external {
        require(amountA > 0 && amountB > 0, "Invalid amounts");

        IERC20(tokenA).transferFrom(msg.sender, address(this), amountA);
        IERC20(tokenB).transferFrom(msg.sender, address(this), amountB);

        reserveA += amountA;
        reserveB += amountB;

        emit LiquidityAdded(msg.sender, amountA, amountB);
    }

    function removeLiquidity(uint256 liquidityPercentage) external {
        require(liquidityPercentage > 0 && liquidityPercentage <= 100, "Invalid percentage");

        uint256 amountA = (reserveA * liquidityPercentage) / 100;
        uint256 amountB = (reserveB * liquidityPercentage) / 100;

        reserveA -= amountA;
        reserveB -= amountB;

        IERC20(tokenA).transfer(msg.sender, amountA);
        IERC20(tokenB).transfer(msg.sender, amountB);

        emit LiquidityRemoved(msg.sender, amountA, amountB);
    }

    function getSwapAmount(uint256 inputAmount, uint256 inputReserve, uint256 outputReserve) public pure returns (uint256) {
        uint256 inputAmountWithFee = inputAmount * (FEE_DENOMINATOR - FEE_PERCENT);
        uint256 numerator = inputAmountWithFee * outputReserve;
        uint256 denominator = (inputReserve * FEE_DENOMINATOR) + inputAmountWithFee;
        return numerator / denominator;
    }

    function swap(address tokenIn, uint256 amountIn) external {
        require(tokenIn == tokenA || tokenIn == tokenB, "Invalid token");
        require(amountIn > 0, "Invalid input amount");

        bool isTokenA = tokenIn == tokenA;

        (uint256 inputReserve, uint256 outputReserve) = isTokenA ? (reserveA, reserveB) : (reserveB, reserveA);

        uint256 amountOut = getSwapAmount(amountIn, inputReserve, outputReserve);
        require(amountOut > 0, "Invalid output amount");

        if (isTokenA) {
            reserveA += amountIn;
            reserveB -= amountOut;
        } else {
            reserveB += amountIn;
            reserveA -= amountOut;
        }

        IERC20(tokenIn).transferFrom(msg.sender, address(this), amountIn);
        IERC20(isTokenA ? tokenB : tokenA).transfer(msg.sender, amountOut);

        emit Swap(msg.sender, tokenIn, amountIn, isTokenA ? tokenB : tokenA, amountOut);
    }
}

interface IERC20 {
    function transfer(address recipient, uint256 amount) external returns (bool);

    function transferFrom(
        address sender,
        address recipient,
        uint256 amount
    ) external returns (bool);
}
