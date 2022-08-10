// https://leetcode.com/problems/fixed-point

pub fn fixed_point(arr: Vec<i32>) -> i32 {
    for i in 0..arr.len() {
        if arr[i] == i as i32 {
            return i as i32;
        }
    }
    return -1;
}

#[test]
pub fn t1() {
    assert_eq!(fixed_point(vec![-10, -5, 0, 3, 7]), 3);
}