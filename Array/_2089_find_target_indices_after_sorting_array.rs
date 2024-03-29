// https://leetcode.com/problems/find-target-indices-after-sorting-array
//
// You are given a **0-indexed** integer array `nums` and a target element `target`.
//
// A **target index** is an index `i` such that `nums[i] == target`.
//
// Return _a list of the target indices of_ `nums` after _sorting_ `nums` _in **non-decreasing** order_. If there are no target indices, return _an **empty** list_. The returned list must be sorted in **increasing** order.
//
// **Example 1:**
//
// ```
// **Input:** nums = [1,2,5,2,3], target = 2
// **Output:** [1,2]
// **Explanation:** After sorting, nums is [1,<u>**2**</u>,<u>**2**</u>,3,5].
// The indices where nums[i] == 2 are 1 and 2.
// ```
//
// **Example 2:**
//
// ```
// **Input:** nums = [1,2,5,2,3], target = 3
// **Output:** [3]
// **Explanation:** After sorting, nums is [1,2,2,<u>**3**</u>,5].
// The index where nums[i] == 3 is 3.
// ```
//
// **Example 3:**
//
// ```
// **Input:** nums = [1,2,5,2,3], target = 5
// **Output:** [4]
// **Explanation:** After sorting, nums is [1,2,2,3,<u>**5**</u>].
// The index where nums[i] == 5 is 4.
// ```
//
// **Constraints:**
//
// *   `1 <= nums.length <= 100`
// *   `1 <= nums[i], target <= 100`

pub fn target_indices(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut sorted_nums = nums.clone();
    sorted_nums.sort();
    return sorted_nums
        .iter()
        .enumerate()
        .filter(|&(_, &num)| num == target)
        .map(|(i, _)| i as i32)
        .collect();
}

#[test]
pub fn t1() {
    assert_eq!(target_indices(vec![1, 2, 5, 2, 3], 2), vec![1, 2]);
}
