// https://leetcode.com/problems/intersection-of-two-arrays
//
// Given two integer arrays `nums1` and `nums2`, return _an array of their intersection_. Each element in the result must be **unique** and you may return the result in **any order**.
//
// **Example 1:**
//
// ```
// **Input:** nums1 = [1,2,2,1], nums2 = [2,2]
// **Output:** [2]
// ```
//
// **Example 2:**
//
// ```
// **Input:** nums1 = [4,9,5], nums2 = [9,4,9,8,4]
// **Output:** [9,4]
// **Explanation:** [4,9] is also accepted.
// ```
//
// **Constraints:**
//
// *   `1 <= nums1.length, nums2.length <= 1000`
// *   `0 <= nums1[i], nums2[i] <= 1000`

pub fn intersection(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
    return nums1
        .iter()
        .cloned()
        .filter(|&x| nums2.contains(&x))
        .collect::<std::collections::HashSet<i32>>()
        .iter()
        .cloned()
        .collect();
}

#[test]
pub fn t1() {
    assert_eq!(intersection(vec![1, 2, 2, 1], vec![2, 2]), vec![2]);
}
