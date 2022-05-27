// https://leetcode.com/problems/coin-change
// 
// You are given an integer array `coins` representing coins of different denominations and an integer `amount` representing a total amount of money.
// 
// Return _the fewest number of coins that you need to make up that amount_. If that amount of money cannot be made up by any combination of the coins, return `-1`.
// 
// You may assume that you have an infinite number of each kind of coin.
// 
// **Example 1:**
// 
// ```
// **Input:** coins = [1,2,5], amount = 11
// **Output:** 3
// **Explanation:** 11 = 5 + 5 + 1
// ```
// 
// **Example 2:**
// 
// ```
// **Input:** coins = [2], amount = 3
// **Output:** -1
// ```
// 
// **Example 3:**
// 
// ```
// **Input:** coins = [1], amount = 0
// **Output:** 0
// ```
// 
// **Constraints:**
// 
// *   `1 <= coins.length <= 12`
// *   `1 <= coins[i] <= 2<sup>31</sup> - 1`
// *   `0 <= amount <= 10<sup>4</sup>`

pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
    let max = (amount + 1) as usize;
    let mut dp = vec![max as i32; max];
    dp[0] = 0;
    for i in 1..=amount {
        for &coin in &coins {
            if coin <= i {
              dp[i as usize] = i32::min(dp[i as usize], dp[(i-coin) as usize] + 1);
            }
        }
    }

    return if dp[amount as usize] > amount {
         -1
    } else {
        dp[amount as usize] as i32
    };
}

#[test]
fn test() {
    assert_eq!(coin_change(vec![1, 2, 5], 11), 3);
}