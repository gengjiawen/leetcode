// https://leetcode.com/problems/smallest-range-i
//
// You are given an integer array `nums` and an integer `k`.
//
// In one operation, you can choose any index `i` where `0 <= i < nums.length` and change `nums[i]` to `nums[i] + x` where `x` is an integer from the range `[-k, k]`. You can apply this operation **at most once** for each index `i`.
//
// The **score** of `nums` is the difference between the maximum and minimum elements in `nums`.
//
// Return _the minimum **score** of_ `nums` _after applying the mentioned operation at most once for each index in it_.
//
// **Example 1:**
//
// ```
// **Input:** nums = [1], k = 0
// **Output:** 0
// **Explanation:** The score is max(nums) - min(nums) = 1 - 1 = 0.
// ```
//
// **Example 2:**
//
// ```
// **Input:** nums = [0,10], k = 2
// **Output:** 6
// **Explanation:** Change nums to be [2, 8]. The score is max(nums) - min(nums) = 8 - 2 = 6.
// ```
//
// **Example 3:**
//
// ```
// **Input:** nums = [1,3,6], k = 3
// **Output:** 0
// **Explanation:** Change nums to be [4, 4, 4]. The score is max(nums) - min(nums) = 4 - 4 = 0.
// ```
//
// **Constraints:**
//
// *   `1 <= nums.length <= 10<sup>4</sup>`
// *   `0 <= nums[i] <= 10<sup>4</sup>`
// *   `0 <= k <= 10<sup>4</sup>`

pub fn smallest_range_i(nums: Vec<i32>, k: i32) -> i32 {
    let mut min_score = nums[0];
    let mut max_score = nums[0];
    for i in 1..nums.len() {
        if nums[i] < min_score {
            min_score = nums[i];
        }
        if nums[i] > max_score {
            max_score = nums[i];
        }
    }

    return 0.max(max_score - min_score - k * 2);
}

#[test]
pub fn t1() {
    assert_eq!(smallest_range_i(vec![1], 0), 0);
    assert_eq!(smallest_range_i(vec![0, 10], 2), 6);
    assert_eq!(smallest_range_i(vec![1, 3, 6], 3), 0);
}
