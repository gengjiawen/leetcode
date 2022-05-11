// https://leetcode.com/problems/minimum-size-subarray-sum/
// Given an array of positive integers nums and a positive integer target,
// return the minimal length of a contiguous subarray [numsl, numsl+1, ..., numsr-1, numsr] of
// which the sum is greater than or equal to target. If there is no such subarray, return 0 instead.
fn min_sub_array_len(target: i32, nums: Vec<i32>) -> i32 {
    let mut start = 0;
    let mut min = std::usize::MAX;
    let mut sum = 0;
    for end in 0..nums.len() {
        sum = sum + nums[end];
        while sum >= target {
            min = min.min(end - start + 1);
            sum = sum - nums[start];
            start += 1;
        }
    }
    if min == std::usize::MAX {
        return 0;
    }
    return min as i32;
}

#[test]
fn test() {
    assert_eq!(min_sub_array_len(7, vec![2, 3, 1, 2, 4, 3]), 2);
    assert_eq!(min_sub_array_len(4, vec![1, 4, 4]), 1);
}
