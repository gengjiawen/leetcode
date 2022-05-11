// https://leetcode.com/problems/contains-duplicate
//
// Given an array of integers, find if the array contains any duplicates.
// Your function should return true if any value appears at least twice in the array,
// and it should return false if every element is distinct.

use std::collections::HashSet;

pub fn contains_duplicate(nums: Vec<i32>) -> bool {
    let mut set = HashSet::new();
    for num in nums {
        if set.contains(&num) {
            return true;
        }
        set.insert(num);
    }
    return false;
}


#[test]
pub fn test() {
    assert_eq!(contains_duplicate(Vec::from([1, 2])), false);
    assert_eq!(contains_duplicate(Vec::from([1, 2, 1])), true);
}
