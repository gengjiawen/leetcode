// https://leetcode.com/problems/maximum-average-subarray-i
// You are given an integer array nums consisting of n elements, and an integer k.

// Find a contiguous subarray whose length is equal to k that has the maximum average value and return this value. Any answer with a calculation error less than 10-5 will be accepted.
// Example 1:

// Input: nums = [1,12,-5,-6,50,3], k = 4
// Output: 12.75000
// Explanation: Maximum average is (12 - 5 - 6 + 50) / 4 = 51 / 4 = 12.75
// Example 2:

// Input: nums = [5], k = 1
// Output: 5.00000

pub fn find_max_average(nums: Vec<i32>, k: i32) -> f64 {
    let mut sum = 0;
    for i in 0..k {
        sum += nums[i as usize];
    }
    let mut max = sum;
    for i in k as usize..nums.len() {
        sum = sum + nums[i] - nums[i - k as usize];
        max = max.max(sum);
    }
    return max as f64 / k as f64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test() {
        assert_eq!(find_max_average(Vec::from([1, 12, -5, -6, 50, 3]), 4), 12.75);
    }
}
