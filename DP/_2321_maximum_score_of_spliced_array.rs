// https://leetcode.com/problems/maximum-score-of-spliced-array
//
// You are given two **0-indexed** integer arrays `nums1` and `nums2`, both of length `n`.
//
// You can choose two integers `left` and `right` where `0 <= left <= right < n` and **swap** the subarray `nums1[left...right]` with the subarray `nums2[left...right]`.
//
// *   For example, if `nums1 = [1,2,3,4,5]` and `nums2 = [11,12,13,14,15]` and you choose `left = 1` and `right = 2`, `nums1` becomes `[1,**<u>12,13</u>**,4,5]` and `nums2` becomes `[11,**<u>2,3</u>**,14,15]`.
//
// You may choose to apply the mentioned operation **once** or not do anything.
//
// The **score** of the arrays is the **maximum** of `sum(nums1)` and `sum(nums2)`, where `sum(arr)` is the sum of all the elements in the array `arr`.
//
// Return _the **maximum possible score**_.
//
// A **subarray** is a contiguous sequence of elements within an array. `arr[left...right]` denotes the subarray that contains the elements of `nums` between indices `left` and `right` (**inclusive**).
//
// **Example 1:**
//
// ```
// **Input:** nums1 = [60,60,60], nums2 = [10,90,10]
// **Output:** 210
// **Explanation:** Choosing left = 1 and right = 1, we have nums1 = [60,<u>**90**</u>,60] and nums2 = [10,<u>**60**</u>,10].
// The score is max(sum(nums1), sum(nums2)) = max(210, 80) = 210.```
//
// **Example 2:**
//
// ```
// **Input:** nums1 = [20,40,20,70,30], nums2 = [50,20,50,40,20]
// **Output:** 220
// **Explanation:** Choosing left = 3, right = 4, we have nums1 = [20,40,20,<u>**40,20**</u>] and nums2 = [50,20,50,<u>**70,30**</u>].
// The score is max(sum(nums1), sum(nums2)) = max(140, 220) = 220.
// ```
//
// **Example 3:**
//
// ```
// **Input:** nums1 = [7,11,13], nums2 = [1,1,1]
// **Output:** 31
// **Explanation:** We choose not to swap any subarray.
// The score is max(sum(nums1), sum(nums2)) = max(31, 3) = 31.
// ```
//
// **Constraints:**
//
// *   `n == nums1.length == nums2.length`
// *   `1 <= n <= 10<sup>5</sup>`
// *   `1 <= nums1[i], nums2[i] <= 10<sup>4</sup>`

pub fn maximums_spliced_array(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
    fn dp(num1: &Vec<i32>, nums2: &Vec<i32>) -> i32 {
        let mut prev_max = 0;
        let mut max = 0;
        for (x, y) in num1.iter().zip(nums2.iter()) {
            prev_max = (y - x).max(y - x + prev_max);
            max = max.max(prev_max);
        }
        return num1.iter().sum::<i32>() + max;
    }

    return dp(&nums1, &nums2).max(dp(&nums2, &nums1));
}

#[test]
pub fn t1() {
    assert_eq!(
        maximums_spliced_array(vec![60, 60, 60], vec![10, 90, 10]),
        210
    );
    assert_eq!(
        maximums_spliced_array(vec![20, 40, 20, 70, 30], vec![50, 20, 50, 40, 20]),
        220
    );
}
