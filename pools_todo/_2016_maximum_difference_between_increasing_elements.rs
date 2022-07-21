// https://leetcode.com/problems/maximum-difference-between-increasing-elements
// 
// Given a **0-indexed** integer array `nums` of size `n`, find the **maximum difference** between `nums[i]` and `nums[j]` (i.e., `nums[j] - nums[i]`), such that `0 <= i < j < n` and `nums[i] < nums[j]`.
// 
// Return _the **maximum difference**._ If no such `i` and `j` exists, return `-1`.
// 
// **Example 1:**
// 
// ```
// **Input:** nums = [7,**<u>1</u>**,**<u>5</u>**,4]
// **Output:** 4
// **Explanation:**
// The maximum difference occurs with i = 1 and j = 2, nums[j] - nums[i] = 5 - 1 = 4.
// Note that with i = 1 and j = 0, the difference nums[j] - nums[i] = 7 - 1 = 6, but i > j, so it is not valid.
// ```
// 
// **Example 2:**
// 
// ```
// **Input:** nums = [9,4,3,2]
// **Output:** -1
// **Explanation:**
// There is no i and j such that i < j and nums[i] < nums[j].
// ```
// 
// **Example 3:**
// 
// ```
// **Input:** nums = [**<u>1</u>**,5,2,**<u>10</u>**]
// **Output:** 9
// **Explanation:**
// The maximum difference occurs with i = 0 and j = 3, nums[j] - nums[i] = 10 - 1 = 9.
// ```
// 
// **Constraints:**
// 
// *   `n == nums.length`
// *   `2 <= n <= 1000`
// *   `1 <= nums[i] <= 10<sup>9</sup>`

pub fn maximum_difference(nums: Vec<i32>) -> i32 {

    }

#[test]
pub fn t1() {
}
