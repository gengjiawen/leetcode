// https://leetcode.com/problems/check-if-an-array-is-consecutive

pub fn is_consecutive(nums: Vec<i32>) -> bool {
    let mut min = nums[0];
    let mut max = nums[0];
    let mut set = std::collections::HashSet::new();
    for i in 1..nums.len() {
        if !(set.insert(&nums[i])) {
            return false;
        }

        if nums[i] < min {
            min = nums[i];
        }
        if nums[i] > max {
            max = nums[i];
        }
    }

    return max - min + 1 == nums.len() as i32;
}

#[test]
pub fn t1() {
    assert_eq!(is_consecutive(vec![1, 2, 3, 4]), true);
}
