// https://leetcode.com/problems/maximum-ascending-subarray-sum
//
// Given an array of positive integers `nums`, return the _maximum possible sum of an **ascending** subarray in_ `nums`.
//
// A subarray is defined as a contiguous sequence of numbers in an array.
//
// A subarray `[nums<sub>l</sub>, nums<sub>l+1</sub>, ..., nums<sub>r-1</sub>, nums<sub>r</sub>]` is **ascending** if for all `i` where `l <= i < r`, `nums<sub>i</sub> < nums<sub>i+1</sub>`. Note that a subarray of size `1` is **ascending**.
//
// **Example 1:**
//
// ```
// **Input:** nums = [10,20,30,5,10,50]
// **Output:** 65
// **Explanation:** [5,10,50] is the ascending subarray with the maximum sum of 65.
// ```
//
// **Example 2:**
//
// ```
// **Input:** nums = [10,20,30,40,50]
// **Output:** 150
// **Explanation:** [10,20,30,40,50] is the ascending subarray with the maximum sum of 150.
// ```
//
// **Example 3:**
//
// ```
// **Input:** nums = [12,17,15,13,10,11,12]
// **Output:** 33
// **Explanation:** [10,11,12] is the ascending subarray with the maximum sum of 33.
// ```
//
// **Constraints:**
//
// *   `1 <= nums.length <= 100`
// *   `1 <= nums[i] <= 100`

pub fn max_ascending_sum(nums: Vec<i32>) -> i32 {
    let mut max_sum = nums[0];
    let mut sum = nums[0];
    for i in 1..nums.len() {
        let num = nums[i];
        if nums[i] > nums[i - 1] {
            sum = std::cmp::max(0, sum + num);
            max_sum = std::cmp::max(max_sum, sum);
        } else {
            sum = num;
        }
    }
    return max_sum;
}

#[test]
pub fn t1() {
    assert_eq!(max_ascending_sum(vec![10, 20, 30, 5, 10, 50]), 65);
    assert_eq!(max_ascending_sum(vec![100, 10, 1]), 100);
}
