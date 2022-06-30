// https://leetcode.com/problems/relative-sort-array
//
// Given two arrays `arr1` and `arr2`, the elements of `arr2` are distinct, and all elements in `arr2` are also in `arr1`.
//
// Sort the elements of `arr1` such that the relative ordering of items in `arr1` are the same as in `arr2`. Elements that do not appear in `arr2` should be placed at the end of `arr1` in **ascending** order.
//
// **Example 1:**
//
// ```
// **Input:** arr1 = [2,3,1,3,2,4,6,7,9,2,19], arr2 = [2,1,4,3,9,6]
// **Output:** [2,2,2,1,4,3,3,9,6,7,19]
// ```
//
// **Example 2:**
//
// ```
// **Input:** arr1 = [28,6,22,8,44,17], arr2 = [22,28,8,6]
// **Output:** [22,28,8,6,17,44]
// ```
//
// **Constraints:**
//
// *   `1 <= arr1.length, arr2.length <= 1000`
// *   `0 <= arr1[i], arr2[i] <= 1000`
// *   All the elements of `arr2` are **distinct**.
// *   Each `arr2[i]` is in `arr1`.

pub fn relative_sort_array(arr1: Vec<i32>, arr2: Vec<i32>) -> Vec<i32> {
    let mut arr1_map = std::collections::HashMap::new();
    for i in 0..arr1.len() {
        *arr1_map.entry(arr1[i]).or_insert(0) += 1;
    }
    let mut arr1_sorted = vec![];
    for i in 0..arr2.len() {
        if arr1_map.contains_key(&arr2[i]) {
            let count = *arr1_map.get(&arr2[i]).unwrap();
            arr1_sorted = [arr1_sorted, vec![arr2[i]; count]].concat();
            arr1_map.remove(&arr2[i]);
        }
    }
    let mut left = vec![];
    for (key, value) in arr1_map {
        left = [left, vec![key; value]].concat();
    }
    left.sort_unstable();
    arr1_sorted = [arr1_sorted, left].concat();
    return arr1_sorted;
}

#[test]
pub fn t1() {
    assert_eq!(
        relative_sort_array(
            vec![2, 3, 1, 3, 2, 4, 6, 7, 9, 2, 19],
            vec![2, 1, 4, 3, 9, 6]
        ),
        vec![2, 2, 2, 1, 4, 3, 3, 9, 6, 7, 19]
    );
}
