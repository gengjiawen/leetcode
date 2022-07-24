// https://leetcode.com/problems/monotonic-array
//
// An array is **monotonic** if it is either monotone increasing or monotone decreasing.
//
// An array `nums` is monotone increasing if for all `i <= j`, `nums[i] <= nums[j]`. An array `nums` is monotone decreasing if for all `i <= j`, `nums[i] >= nums[j]`.
//
// Given an integer array `nums`, return `true` _if the given array is monotonic, or_ `false` _otherwise_.
//
// **Example 1:**
//
// ```
// **Input:** nums = [1,2,2,3]
// **Output:** true
// ```
//
// **Example 2:**
//
// ```
// **Input:** nums = [6,5,4,4]
// **Output:*true
// ```
//
// **Example 3:**
//
// ```
// **Input:** nums = [1,3,2]
// **Output:** false
// ```
//
// **Constraints:**
//
// *   `1 <= nums.length <= 10<sup>5</sup>`
// *   `-10<sup>5</sup> <= nums[i] <= 10<sup>5</sup>`

pub fn is_monotonic(nums: Vec<i32>) -> bool {
    let mut initialize = false;
    let mut is_grow = false;
    for i in 1..nums.len() {
        if nums[i] > nums[i - 1] {
            if initialize {
                if !is_grow {
                    return false;
                }
            } else {
                initialize = true;
                is_grow = true
            }
        } else if nums[i] < nums[i - 1] {
            if initialize {
                if is_grow {
                    return false;
                }
            } else {
                initialize = true;
                is_grow = false
            }
        }
    }
    return true;
}

#[test]
pub fn t1() {
    assert_eq!(is_monotonic(vec![1, 2, 2, 3]), true);
    assert_eq!(is_monotonic(vec![6, 5, 4, 4]), true);
    assert_eq!(is_monotonic(vec![1, 3, 2]), false);
}
