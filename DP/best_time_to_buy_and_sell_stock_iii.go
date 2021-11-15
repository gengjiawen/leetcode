// https://leetcode.com/problems/best-time-to-buy-and-sell-stock-iii
//
// Say you have an array for which the _i_<sup>th</sup> element is the price of a given stock on day _i_.
//
// Design an algorithm to find the maximum profit. You may complete at most _two_ transactions.
//
// **Note:**
// You may not engage in multiple transactions at the same time (ie, you must sell the stock before you buy again).
package DP

func Min(x, y int) int {
	if x > y {
		return y
	}
	return x
}

func Max(x, y int) int {
	if x < y {
		return y
	}
	return x
}

/**
find all best profits in one traction, then consider two saction, plus firstHalf and SecondHalf
*/
func maxProfit3(prices []int) int {
	priceLen := len(prices)
	if priceLen < 2 {
		return 0
	}

	profits := make([]int, priceLen)
	minP := prices[0]

	for i := 1; i < priceLen; i++ {
		minP = Min(minP, prices[i-1])
		profits[i] = Max(profits[i-1], prices[i]-minP)
	}

	maxProfit := profits[priceLen-1]
	secondMax := prices[priceLen-1]
	secondProfit := 0

	for i := priceLen - 2; i >= 1; i-- {
		secondMax = Max(secondMax, prices[i+1])
		secondProfit = Max(secondProfit, secondMax-prices[i])
		firstProfit := profits[i-1]
		maxProfit = Max(maxProfit, firstProfit+secondProfit)
	}

	return maxProfit
}
