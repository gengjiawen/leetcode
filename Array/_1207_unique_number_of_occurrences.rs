// https://leetcode.com/problems/unique-number-of-occurrences
//
// Given an array of integers `arr`, return `true` if the number of occurrences of each value in the array is **unique**, or `false` otherwise.
//
// **Example 1:**
//
// ```
// **Input:** arr = [1,2,2,1,1,3]
// **Output:** true
// **Explanation:** The value 1 has 3 occurrences, 2 has 2 and 3 has 1\. No two values have the same number of occurrences.```
//
// **Example 2:**
//
// ```
// **Input:** arr = [1,2]
// **Output:** false
// ```
//
// **Example 3:**
//
// ```
// **Input:** arr = [-3,0,1,-3,1,1,1,-3,10,0]
// **Output:** true
// ```
//
// **Constraints:**
//
// *   `1 <= arr.length <= 1000`
// *   `-1000 <= arr[i] <= 1000`

pub fn unique_occurrences(arr: Vec<i32>) -> bool {
    let map = arr
        .iter()
        .fold(std::collections::HashMap::new(), |mut acc, &x| {
            *acc.entry(x).or_insert(0) += 1;
            acc
        });
    return map
        .clone()
        .into_values()
        .collect::<std::collections::HashSet<i32>>()
        .len()
        == map.keys().len();
}

#[test]
pub fn t1() {
    assert_eq!(unique_occurrences(vec![1, 2, 2, 1, 1, 3]), true);
    assert_eq!(unique_occurrences(vec![1, 2]), false);
}
