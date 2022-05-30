// https://leetcode.com/problems/max-consecutive-ones-ii
// Given a binary array nums, return the maximum number of consecutive 1's in the array if you can flip at most one 0.

pub fn longest_ones(nums: Vec<i32>) -> i32 {
    let mut start = 0;

    let mut max_len = 0;
    let mut map = std::collections::HashMap::new();

    for end in 0..nums.len() {
        let num = nums[end];
        *map.entry(num).or_insert(0) += 1;
        if *map.entry(0).or_insert(0) <= 1 {
            max_len = max_len.max(end - start + 1);
        }

        while *map.entry(0).or_insert(0) > 1 {
            let head_val = nums[start];
            *map.entry(head_val).or_insert(0) -= 1;
            start += 1;
        }
    }

    return max_len as i32;
}

#[test]
pub fn test() {
    assert_eq!(longest_ones(vec![1, 0, 1, 1, 0]), 4);
}
