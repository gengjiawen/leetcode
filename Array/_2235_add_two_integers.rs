// https://leetcode.com/problems/add-two-integers
//
// Given two integers `num1` and `num2`, return _the **sum** of the two integers_.
//
// **Example 1:**
//
// ```
// **Input:** num1 = 12, num2 = 5
// **Output:** 17
// **Explanation:** num1 is 12, num2 is 5, and their sum is 12 + 5 = 17, so 17 is returned.
// ```
//
// **Example 2:**
//
// ```
// **Input:** num1 = -10, num2 = 4
// **Output:** -6
// **Explanation:** num1 + num2 = -6, so -6 is returned.
// ```
//
// **Constraints:**
//
// *   `-100 <= num1, num2 <= 100`

pub fn sum(num1: i32, num2: i32) -> i32 {
    return num1 + num2;
}

#[test]
pub fn t1() {
    assert_eq!(sum(12, 5), 17);
    assert_eq!(sum(-10, 4), -6);
}
