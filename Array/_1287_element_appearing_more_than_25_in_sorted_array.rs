// https://leetcode.com/problems/element-appearing-more-than-25-in-sorted-array
//
// Given an integer array **sorted** in non-decreasing order,
// there is exactly one integer in the array that occurs more than 25% of the time, return that integer.
//
// **Example 1:**
//
// ```
// **Input:** arr = [1,2,2,6,6,6,6,7,10]
// **Output:** 6
// ```
//
// **Example 2:**
//
// ```
// **Input:** arr = [1,1]
// **Output:** 1
// ```
//
// **Constraints:**
//
// *   `1 <= arr.length <= 10<sup>4</sup>`
// *   `0 <= arr[i] <= 10<sup>5</sup>`

pub fn find_special_integer(arr: Vec<i32>) -> i32 {
    return arr
        .iter()
        .fold(std::collections::HashMap::new(), |mut acc, &x| {
            *acc.entry(x).or_insert(0) += 1;
            acc
        })
        .into_iter()
        .max_by_key(|&(_, v)| v)
        .unwrap()
        .0;
}

#[test]
pub fn t1() {
    assert_eq!(find_special_integer(vec![1, 2, 2, 6, 6, 6, 6, 7, 10]), 6);
    assert_eq!(find_special_integer(vec![1, 1]), 1);
}
