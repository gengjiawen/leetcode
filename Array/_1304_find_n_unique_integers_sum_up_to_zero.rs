// https://leetcode.com/problems/find-n-unique-integers-sum-up-to-zero
//
// Given an integer `n`, return **any** array containing `n` **unique** integers such that they add up to `0`.
//
// **Example 1:**
//
// ```
// **Input:** n = 5
// **Output:** [-7,-1,1,3,4]
// **Explanation:** These arrays also are accepted [-5,-1,1,2,3] , [-3,-1,2,-2,4].
// ```
//
// **Example 2:**
//
// ```
// **Input:** n = 3
// **Output:** [-1,0,1]
// ```
//
// **Example 3:**
//
// ```
// **Input:** n = 1
// **Output:** [0]
// ```
//
// **Constraints:**
//
// *   `1 <= n <= 1000`

pub fn sum_zero(n: i32) -> Vec<i32> {
    // (1 - n..n).step_by(2).collect()
    return (1..n).chain(std::iter::once(-n * (n - 1) / 2)).collect();
}

#[test]
pub fn t1() {
    println!("{:?}", sum_zero(4));
    println!("{:?}", sum_zero(5));
    assert_eq!(sum_zero(4).iter().sum::<i32>(), 0);
    assert_eq!(sum_zero(5).iter().sum::<i32>(), 0);
}
