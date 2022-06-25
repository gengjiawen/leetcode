// https://leetcode.com/problems/number-of-valid-subarrays

pub fn valid_subarrays(nums: Vec<i32>) -> i32 {
    let mut result = 0;
    for i in 0..nums.len() {
        for j in i..nums.len() {
            if nums[j] >= nums[i] {
                result += 1;
            } else {
                break;
            }
        }
    }
    return result;
}

#[test]
pub fn t1() {
    assert_eq!(valid_subarrays(vec![1, 4, 2, 5, 3]), 11);
    assert_eq!(valid_subarrays(vec![2, 2, 2]), 6);
}
