// https://leetcode.com/problems/largest-perimeter-triangle
//
// Given an integer array `nums`, return _the largest perimeter of a triangle with a non-zero area, formed from three of these lengths_. If it is impossible to form any triangle of a non-zero area, return `0`.
//
// **Example 1:**
//
// ```
// **Input:** nums = [2,1,2]
// **Output:** 5
// ```
//
// **Example 2:**
//
// ```
// **Input:** nums = [1,2,1]
// **Output:** 0
// ```
//
// **Constraints:**
//
// *   `3 <= nums.length <= 10<sup>4</sup>`
// *   `1 <= nums[i] <= 10<sup>6</sup>`

pub fn largest_perimeter(nums: Vec<i32>) -> i32 {
    let mut nums_clone = nums.clone();
    nums_clone.sort_unstable();
    for i in (0..nums_clone.len()).rev() {
        if i < 2 {
            break;
        }
        if nums_clone[i] < nums_clone[i - 1] + nums_clone[i - 2] {
            return nums_clone[i] + nums_clone[i - 1] + nums_clone[i - 2];
        }
    }
    return 0;
}

#[test]
pub fn t1() {
    assert_eq!(largest_perimeter(vec![2, 1, 2]), 5);
}
