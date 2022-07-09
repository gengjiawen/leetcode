// https://leetcode.com/problems/two-out-of-three
//
// Given three integer arrays `nums1`, `nums2`, and `nums3`, return _a **distinct** array containing all the values that are present in **at least two** out of the three arrays. You may return the values in **any** order_.
//
// **Example 1:**
//
// ```
// **Input:** nums1 = [1,1,3,2], nums2 = [2,3], nums3 = [3]
// **Output:** [3,2]
// **Explanation:** The values that are present in at least two arrays are:
// - 3, in all three arrays.
// - 2, in nums1 and nums2.
// ```
//
// **Example 2:**
//
// ```
// **Input:** nums1 = [3,1], nums2 = [2,3], nums3 = [1,2]
// **Output:** [2,3,1]
// **Explanation:** The values that are present in at least two arrays are:
// - 2, in nums2 and nums3.
// - 3, in nums1 and nums2.
// - 1, in nums1 and nums3.
// ```
//
// **Example 3:**
//
// ```
// **Input:** nums1 = [1,2,2], nums2 = [4,3,3], nums3 = [5]
// **Output:** []
// **Explanation:** No value is present in at least two arrays.
// ```
//
// **Constraints:**
//
// *   `1 <= nums1.length, nums2.length, nums3.length <= 100`
// *   `1 <= nums1[i], nums2[j], nums3[k] <= 100`

pub fn two_out_of_three(nums1: Vec<i32>, nums2: Vec<i32>, nums3: Vec<i32>) -> Vec<i32> {
    let mut res = std::collections::HashSet::new();
    for num in nums1 {
        if nums2.contains(&num) || nums3.contains(&num) {
            res.insert(num);
        }
    }
    for num in nums2 {
        if nums3.contains(&num) {
            res.insert(num);
        }
    }

    return res.into_iter().collect();
}

#[test]
pub fn t1() {
    assert_eq!(
        two_out_of_three(vec![1, 1, 3, 2], vec![2, 3], vec![3]),
        vec![3, 2]
    );
}
