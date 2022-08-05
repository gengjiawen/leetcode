// https://leetcode.com/problems/check-if-all-1s-are-at-least-length-k-places-away
//
// Given an binary array `nums` and an integer `k`, return `true` _if all_ `1`_'s are at least_ `k` _places away from each other, otherwise return_ `false`.
//
// **Example 1:**
//
// ![](https://assets.leetcode.com/uploads/2020/04/15/sample_1_1791.png)
// ```
// **Input:** nums = [1,0,0,0,1,0,0,1], k = 2
// **Output:** true
// **Explanation:** Each of the 1s are at least 2 places away from each other.
// ```
//
// **Example 2:**
//
// ![](https://assets.leetcode.com/uploads/2020/04/15/sample_2_1791.png)
// ```
// **Input:** nums = [1,0,0,1,0,1], k = 2
// **Output:** false
// **Explanation:** The second 1 and third 1 are only one apart from each other.
// ```
//
// **Constraints:**
//
// *   `1 <= nums.length <= 10<sup>5</sup>`
// *   `0 <= k <= nums.length`
// *   `nums[i]` is `0` or `1`

pub fn k_length_apart(nums: Vec<i32>, k: i32) -> bool {
    let mut start_index = -k - 1;
    for i in 0..nums.len() {
        if nums[i] == 1 {
            if i as i32 - start_index - 1 < k {
                return false;
            }
            start_index = i as i32;
        }
    }
    return true;
}

#[test]
pub fn t1() {
    assert_eq!(k_length_apart(vec![1, 0, 0, 0, 1, 0, 0, 1], 2), true);
    assert_eq!(k_length_apart(vec![1, 0, 0, 1, 0, 1], 2), false);
}
