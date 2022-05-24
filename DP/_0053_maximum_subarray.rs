// https://leetcode.com/problems/maximum-subarray
// 
// Given an integer array `nums`, find the contiguous subarray (containing at least one number) which has the largest sum and return _its sum_.
// 
// A **subarray** is a **contiguous** part of an array.
// 
// **Example 1:**
// 
// ```
// **Input:** nums = [-2,1,-3,4,-1,2,1,-5,4]
// **Output:** 6
// **Explanation:** [4,-1,2,1] has the largest sum = 6.
// ```
// 
// **Example 2:**
// 
// ```
// **Input:** nums = [1]
// **Output:** 1
// ```
// 
// **Example 3:**
// 
// ```
// **Input:** nums = [5,4,-1,7,8]
// **Output:** 23
// ```
// 
// **Constraints:**
// 
// *   `1 <= nums.length <= 10<sup>5</sup>`
// *   `-10<sup>4</sup> <= nums[i] <= 10<sup>4</sup>`
// 
// **Follow up:** If you have figured out the `O(n)` solution, try coding another solution using the **divide and conquer** approach, which is more subtle.

pub fn max_sub_array(nums: Vec<i32>) -> i32 {
    let mut prev_max = 0;
    let mut max = std::i32::MIN;
    for i in 0..nums.len() {
        prev_max = nums[i].max(nums[i] + prev_max);
        max = max.max(prev_max);
    }
    return max
}

#[test]
fn test() {
    assert_eq!(max_sub_array(vec![5,4,-1,7,8]), 23);
}