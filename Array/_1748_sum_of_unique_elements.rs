// https://leetcode.com/problems/sum-of-unique-elements
//
// You are given an integer array `nums`. The unique elements of an array are the elements that appear **exactly once** in the array.
//
// Return _the **sum** of all the unique elements of_ `nums`.
//
// **Example 1:**
//
// ```
// **Input:** nums = [1,2,3,2]
// **Output:** 4
// **Explanation:** The unique elements are [1,3], and the sum is 4.
// ```
//
// **Example 2:**
//
// ```
// **Input:** nums = [1,1,1,1,1]
// **Output:** 0
// **Explanation:** There are no unique elements, and the sum is 0.
// ```
//
// **Example 3:**
//
// ```
// **Input:** nums = [1,2,3,4,5]
// **Output:** 15
// **Explanation:** The unique elements are [1,2,3,4,5], and the sum is 15.
// ```
//
// **Constraints:**
//
// *   `1 <= nums.length <= 100`
// *   `1 <= nums[i] <= 100`

pub fn sum_of_unique(nums: Vec<i32>) -> i32 {
    return nums
        .iter()
        .fold(std::collections::HashMap::new(), |mut acc, x| {
            *acc.entry(x).or_insert(0) += 1;
            acc
        })
        .into_iter()
        .filter(|(_, v)| *v == 1)
        .fold(0, |acc, (k, _)| acc + k);
}

#[test]
pub fn t1() {
    assert_eq!(sum_of_unique(vec![1, 2, 3, 2]), 4);
}
