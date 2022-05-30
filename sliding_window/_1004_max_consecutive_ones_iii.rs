// https://leetcode.com/problems/max-consecutive-ones-iii
//
// Given a binary array `nums` and an integer `k`, return _the maximum number of consecutive_ `1`_'s in the array if you can flip at most_ `k` `0`'s.
//
// **Example 1:**
//
// ```
// **Input:** nums = [1,1,1,0,0,0,1,1,1,1,0], k = 2
// **Output:** 6
// **Explanation:** [1,1,1,0,0,<u>**1**,1,1,1,1,**1**</u>]
// Bolded numbers were flipped from 0 to 1\. The longest subarray is underlined.```
//
// **Example 2:**
//
// ```
// **Input:** nums = [0,0,1,1,0,0,1,1,1,0,1,1,0,0,0,1,1,1,1], k = 3
// **Output:** 10
// **Explanation:** [0,0,<u>1,1,**1**,**1**,1,1,1,**1**,1,1</u>,0,0,0,1,1,1,1]
// Bolded numbers were flipped from 0 to 1\. The longest subarray is underlined.
// ```
//
// **Constraints:**
//
// *   `1 <= nums.length <= 10<sup>5</sup>`
// *   `nums[i]` is either `0` or `1`.
// *   `0 <= k <= nums.length`

pub fn longest_ones(nums: Vec<i32>, k: i32) -> i32 {
    let mut start = 0;

    let mut max_len = 0;
    let mut map = std::collections::HashMap::new();

    for end in 0..nums.len() {
        let num = nums[end];
        *map.entry(num).or_insert(0) += 1;
        if *map.entry(0).or_insert(0) <= k {
            max_len = max_len.max(end - start + 1);
        }

        while *map.entry(0).or_insert(0) > k {
            let head_val = nums[start];
            *map.entry(head_val).or_insert(0) -= 1;
            start += 1;
        }
    }

    return max_len as i32;
}

#[test]
pub fn test() {
    assert_eq!(longest_ones(vec![1, 1, 1, 0, 0, 0, 1, 1, 1, 1, 0], 2), 6);
}
