// https://leetcode.com/problems/check-if-a-number-is-majority-element-in-a-sorted-array

pub fn is_majority_element(nums: Vec<i32>, target: i32) -> bool {
    return nums.iter().filter(|&x| *x == target).count() > nums.len() / 2;
}

#[test]
pub fn t1() {
    assert_eq!(is_majority_element(vec![2, 2, 1, 1, 1, 2, 2], 2), true);
}
