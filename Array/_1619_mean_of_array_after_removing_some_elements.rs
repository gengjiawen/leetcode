// https://leetcode.com/problems/mean-of-array-after-removing-some-elements
//
// Given an integer array `arr`, return _the mean of the remaining integers after removing the smallest `5%` and the largest `5%` of the elements._
//
// Answers within `10<sup>-5</sup>` of the **actual answer** will be considered accepted.
//
// **Example 1:**
//
// ```
// **Input:** arr = [1,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,3]
// **Output:** 2.00000
// **Explanation:** After erasing the minimum and the maximum values of this array, all elements are equal to 2, so the mean is 2.
// ```
//
// **Example 2:**
//
// ```
// **Input:** arr = [6,2,7,5,1,2,0,3,10,2,5,0,5,5,0,8,7,6,8,0]
// **Output:** 4.00000
// ```
//
// **Example 3:**
//
// ```
// **Input:** arr = [6,0,7,0,7,5,7,8,3,4,0,7,8,1,6,8,1,1,2,4,8,1,9,5,4,3,8,5,10,8,6,6,1,0,6,10,8,2,3,4]
// **Output:** 4.77778
// ```
//
// **Constraints:**
//
// *   `20 <= arr.length <= 1000`
// *   `arr.length`**is a multiple** of `20`.
// *   `<font face="monospace">0 <= arr[i] <= 10<sup>5</sup></font>`

pub fn trim_mean(arr: Vec<i32>) -> f64 {
    let mut sorted_arr = arr;
    sorted_arr.sort_unstable();
    let drop_count = (sorted_arr.len() as f64 * 0.05) as usize;
    let mut sum = 0;
    for i in drop_count..sorted_arr.len() - drop_count {
        sum += sorted_arr[i];
    }
    return sum as f64 / (sorted_arr.len() - 2 * drop_count) as f64;
}

#[test]
pub fn t1() {
    assert_eq!(
        trim_mean(vec![
            1, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 3
        ]),
        2.0
    );
}
