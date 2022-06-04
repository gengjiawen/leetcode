// https://leetcode.com/problems/sum-of-digits-in-the-minimum-number

pub fn sum_of_digits(nums: Vec<i32>) -> i32 {
    let n = nums.iter().min().unwrap();
    return 1 - (n / 100 + n / 10 % 10 + n % 10) % 2;
}

#[test]
pub fn t1() {
    assert_eq!(sum_of_digits(vec![99, 77, 33, 66, 55]), 1);
}
