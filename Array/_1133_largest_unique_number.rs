// https://leetcode.com/problems/largest-unique-number

pub fn largest_unique_number(nums: Vec<i32>) -> i32 {
    let mut btm = std::collections::BTreeMap::new();
    for i in nums {
        *btm.entry(i).or_insert(0) += 1;
    }

    for (k, v) in btm.iter().rev() {
        if *v == 1 {
            return *k;
        }
    }
    return -1;
}

#[test]
pub fn t1() {
    assert_eq!(largest_unique_number(vec![5, 7, 3, 9, 4, 9, 8, 3, 1]), 8);
}
