// https://leetcode.com/problems/count-number-of-pairs-with-absolute-difference-k
// 
// Given an integer array `nums` and an integer `k`, return _the number of pairs_ `(i, j)` _where_ `i < j` _such that_ `|nums[i] - nums[j]| == k`.
// 
// The value of `|x|` is defined as:
// 
// *   `x` if `x >= 0`.
// *   `-x` if `x < 0`.
// 
// **Example 1:**
// 
// ```
// **Input:** nums = [1,2,2,1], k = 1
// **Output:** 4
// **Explanation:** The pairs with an absolute difference of 1 are:
// - [**<u>1</u>**,**<u>2</u>**,2,1]
// - [**<u>1</u>**,2,**<u>2</u>**,1]
// - [1,**<u>2</u>**,2,**<u>1</u>**]
// - [1,2,**<u>2</u>**,**<u>1</u>**]
// ```
// 
// **Example 2:**
// 
// ```
// **Input:** nums = [1,3], k = 3
// **Output:** 0
// **Explanation:** There are no pairs with an absolute difference of 3.
// ```
// 
// **Example 3:**
// 
// ```
// **Input:** nums = [3,2,1,5,4], k = 2
// **Output:** 3
// **Explanation:** The pairs with an absolute difference of 2 are:
// - [**<u>3</u>**,2,**<u>1</u>**,5,4]
// - [**<u>3</u>**,2,1,**<u>5</u>**,4]
// - [3,**<u>2</u>**,1,5,**<u>4</u>**]
// ```
// 
// **Constraints:**
// 
// *   `1 <= nums.length <= 200`
// *   `1 <= nums[i] <= 100`
// *   `1 <= k <= 99`

pub fn count_k_difference(nums: Vec<i32>, k: i32) -> i32 {

    }

#[test]
pub fn t1() {
}
