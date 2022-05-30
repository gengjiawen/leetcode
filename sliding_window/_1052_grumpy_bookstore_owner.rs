// https://leetcode.com/problems/grumpy-bookstore-owner
//
// There is a bookstore owner that has a store open for `n` minutes.
// Every minute, some number of customers enter the store.
// You are given an integer array `customers` of length `n` where `customers[i]` is the number of
// the customer that enters the store at the start of the `i<sup>th</sup>`
// minute and all those customers leave after the end of that minute.
//
// On some minutes, the bookstore owner is grumpy.
// You are given a binary array grumpy where `grumpy[i]` is `1` if the bookstore owner is grumpy during the `i<sup>th</sup>` minute, and is `0` otherwise.
//
// When the bookstore owner is grumpy, the customers of that minute are not satisfied, otherwise, they are satisfied.
//
// The bookstore owner knows a secret technique to keep themselves not grumpy for `minutes` consecutive minutes, but can only use it once.
//
// Return _the maximum number of customers that can be satisfied throughout the day_.
//
// **Example 1:**
//
// ```
// **Input:** customers = [1,0,1,2,1,1,7,5], grumpy = [0,1,0,1,0,1,0,1], minutes = 3
// **Output:** 16
// **Explanation:** The bookstore owner keeps themselves not grumpy for the last 3 minutes.
// The maximum number of customers that can be satisfied = 1 + 1 + 1 + 1 + 7 + 5 = 16.
// ```
//
// **Example 2:**
//
// ```
// **Input:** customers = [1], grumpy = [0], minutes = 1
// **Output:** 1
// ```
//
// **Constraints:**
//
// *   `n == customers.length == grumpy.length`
// *   `1 <= minutes <= n <= 2 * 10<sup>4</sup>`
// *   `0 <= customers[i] <= 1000`
// *   `grumpy[i]` is either `0` or `1`.

pub fn max_satisfied(customers: Vec<i32>, grumpy: Vec<i32>, minutes: i32) -> i32 {
    let mut always_satisfied = 0;
    let mut tmp = 0;
    let mut extra = 0;
    let mut start = 0;
    for end in 0..customers.len() {
        always_satisfied += (1 - grumpy[end]) * customers[end]; // always satisfied customer

        // keep sliding window customer satisfied when store owner is grumpy
        tmp += customers[end] * grumpy[end];
        if end - start == minutes as usize {
            tmp -= customers[start] * grumpy[start];
            start += 1;
        }
        extra = extra.max(tmp);
    }

    return always_satisfied + extra;
}

#[test]
pub fn test() {
    assert_eq!(
        max_satisfied(
            vec![1, 0, 1, 2, 1, 1, 7, 5],
            vec![0, 1, 0, 1, 0, 1, 0, 1],
            3
        ),
        16
    );
    assert_eq!(max_satisfied(vec![1], vec![0], 1), 1);
    assert_eq!(max_satisfied(vec![2, 6, 6, 9], vec![0, 0, 1, 1], 1), 17);
}
