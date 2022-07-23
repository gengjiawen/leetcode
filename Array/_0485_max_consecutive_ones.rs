// https://leetcode.com/problems/max-consecutive-ones
//
// Given a binary array `nums`, return _the maximum number of consecutive_ `1`_'s in the array_.
//
// **Example 1:**
//
// ```
// **Input:** nums = [1,1,0,1,1,1]
// **Output:** 3
// **Explanation:** The first two digits or the last three digits are consecutive 1s. The maximum number of consecutive 1s is 3.
// ```
//
// **Example 2:**
//
// ```
// **Input:** nums = [1,0,1,1,0,1]
// **Output:** 2
// ```
//
// **Constraints:**
//
// *   `1 <= nums.length <= 10<sup>5</sup>`
// *   `nums[i]` is either `0` or `1`.

pub fn find_max_consecutive_ones(nums: Vec<i32>) -> i32 {
    let mut max = 0;
    let mut count = 0;
    for i in 0..nums.len() {
        if nums[i] == 1 {
            count += 1;
        } else {
            if count > max {
                max = count;
            }
            count = 0;
        }
    }
    if count > max {
        max = count;
    }
    return max;
}

#[test]
pub fn t1() {
    assert_eq!(find_max_consecutive_ones(vec![1, 1, 0, 1, 1, 1]), 3);
}
