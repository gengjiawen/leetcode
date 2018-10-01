// https://leetcode.com/problems/best-time-to-buy-and-sell-stock-ii
//
// Say you have an array for which the _i_<sup>th</sup> element is the price of a given stock on day _i_.
//
// Design an algorithm to find the maximum profit. You may complete as many transactions as you like (ie, buy one and sell one share of the stock multiple times). However, you may not engage in multiple transactions at the same time (ie, you must sell the stock before you buy again).
package main

func maxProfit(prices []int) int {
	if len(prices) <= 1 {
		return 0
	}
	profit := 0

	for i := 1; i < len(prices); i++ {
		if prices[i]-prices[i-1] > 0 {
			profit += prices[i] - prices[i-1]
		}
	}

	return profit
}
