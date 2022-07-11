// https://leetcode.com/problems/find-all-numbers-disappeared-in-an-array
//
// Given an array `nums` of `n` integers where `nums[i]` is in the range `[1, n]`, return _an array of all the integers in the range_ `[1, n]` _that do not appear in_ `nums`.
//
// **Example 1:**
//
// ```
// **Input:** nums = [4,3,2,7,8,2,3,1]
// **Output:** [5,6]
// ```
//
// **Example 2:**
//
// ```
// **Input:** nums = [1,1]
// **Output:** [2]
// ```
//
// **Constraints:**
//
// *   `n == nums.length`
// *   `1 <= n <= 10<sup>5</sup>`
// *   `1 <= nums[i] <= n`
//
// **Follow up:** Could you do it without extra space and in `O(n)` runtime? You may assume the returned list does not count as extra space.

pub fn find_disappeared_numbers(nums: Vec<i32>) -> Vec<i32> {
    let mut sets = (1..(nums.len() + 1) as i32)
        .into_iter()
        .collect::<std::collections::HashSet<i32>>();
    for num in nums {
        sets.remove(&num);
    }
    return sets.into_iter().collect();
}

#[test]
pub fn t1() {
    assert_eq!(
        find_disappeared_numbers(vec![4, 3, 2, 7, 8, 2, 3, 1]),
        vec![5, 6]
    );
    assert_eq!(find_disappeared_numbers(vec![1, 1]), vec![2]);
}
