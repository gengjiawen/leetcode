// https://leetcode.com/problems/minimum-absolute-difference
//
// Given an array of **distinct** integers `arr`, find all pairs of elements with the minimum absolute difference of any two elements.
//
// Return a list of pairs in ascending order(with respect to pairs), each pair `[a, b]` follows
//
// *   `a, b` are from `arr`
// *   `a < b`
// *   `b - a` equals to the minimum absolute difference of any two elements in `arr`
//
// **Example 1:**
//
// ```
// **Input:** arr = [4,2,1,3]
// **Output:** [[1,2],[2,3],[3,4]]
// **Explanation:** The minimum absolute difference is 1\. List all pairs with difference equal to 1 in ascending order.```
//
// **Example 2:**
//
// ```
// **Input:** arr = [1,3,6,10,15]
// **Output:** [[1,3]]
// ```
//
// **Example 3:**
//
// ```
// **Input:** arr = [3,8,-10,23,19,-4,-14,27]
// **Output:** [[-14,-10],[19,23],[23,27]]
// ```
//
// **Constraints:**
//
// *   `2 <= arr.length <= 10<sup>5</sup>`
// *   `-10<sup>6</sup> <= arr[i] <= 10<sup>6</sup>`

pub fn minimum_abs_difference(arr: Vec<i32>) -> Vec<Vec<i32>> {
    let mut arr_sorted = arr.clone();
    arr_sorted.sort_unstable();
    let mut min_diff = std::i32::MAX;
    let mut result = vec![];
    for i in 0..arr_sorted.len() - 1 {
        let diff = arr_sorted[i + 1] - arr_sorted[i];
        if diff == min_diff {
            result.push(vec![arr_sorted[i], arr_sorted[i + 1]]);
        }
        if diff < min_diff {
            min_diff = diff;
            result = vec![vec![arr_sorted[i], arr_sorted[i + 1]]];
        }
    }
    return result;
}

#[test]
pub fn t1() {
    assert_eq!(
        minimum_abs_difference(vec![4, 2, 1, 3]),
        vec![vec![1, 2], vec![2, 3], vec![3, 4]]
    );
}
