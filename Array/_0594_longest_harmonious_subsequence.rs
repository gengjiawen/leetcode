// https://leetcode.com/problems/longest-harmonious-subsequence
//
// We define a harmonious array as an array where the difference between its maximum value and its minimum value is **exactly** `1`.
//
// Given an integer array `nums`, return _the length of its longest harmonious subsequence among all its possible subsequences_.
//
// A **subsequence** of array is a sequence that can be derived from the array by deleting some or no elements without changing the order of the remaining elements.
//
// **Example 1:**
//
// ```
// **Input:** nums = [1,3,2,2,5,2,3,7]
// **Output:** 5
// **Explanation:** The longest harmonious subsequence is [3,2,2,2,3].
// ```
//
// **Example 2:**
//
// ```
// **Input:** nums = [1,2,3,4]
// **Output:** 2
// ```
//
// **Example 3:**
//
// ```
// **Input:** nums = [1,1,1,1]
// **Output:** 0
// ```
//
// **Constraints:**
//
// *   `1 <= nums.length <= 2 * 10<sup>4</sup>`
// *   `-10<sup>9</sup> <= nums[i] <= 10<sup>9</sup>`

pub fn find_lhs(nums: Vec<i32>) -> i32 {
    let nums_map = nums
        .iter()
        .fold(std::collections::HashMap::new(), |mut acc, &x| {
            *acc.entry(x).or_insert(0) += 1;
            acc
        });
    let mut res = 0;
    for (&k, &v) in &nums_map {
        if let Some(&v2) = nums_map.get(&(k + 1)) {
            res = std::cmp::max(res, v + v2);
        }
    }
    return res;
}

#[test]
pub fn t1() {
    assert_eq!(find_lhs(vec![1, 3, 2, 2, 5, 2, 3, 7]), 5);
}
