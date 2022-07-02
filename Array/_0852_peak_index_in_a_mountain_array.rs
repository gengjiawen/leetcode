// https://leetcode.com/problems/peak-index-in-a-mountain-array
//
// Let's call an array `arr` a **mountain** if the following properties hold:
//
// *   `arr.length >= 3`
// *   There exists some `i` with `0 < i < arr.length - 1` such that:
//     *   `arr[0] < arr[1] < ... arr[i-1] < arr[i]`
//     *   `arr[i] > arr[i+1] > ... > arr[arr.length - 1]`
//
// Given an integer array `arr` that is **guaranteed** to be a mountain, return any `i` such that `arr[0] < arr[1] < ... arr[i - 1] < arr[i] > arr[i + 1] > ... > arr[arr.length - 1]`.
//
// **Example 1:**
//
// ```
// **Input:** arr = [0,1,0]
// **Output:** 1
// ```
//
// **Example 2:**
//
// ```
// **Input:** arr = [0,2,1,0]
// **Output:** 1
// ```
//
// **Example 3:**
//
// ```
// **Input:** arr = [0,10,5,2]
// **Output:** 1
// ```
//
// **Constraints:**
//
// *   `3 <= arr.length <= 10<sup>4</sup>`
// *   `0 <= arr[i] <= 10<sup>6</sup>`
// *   `arr` is **guaranteed** to be a mountain array.
//
// **Follow up:** Finding the `O(n)` is straightforward, could you find an `O(log(n))` solution?

pub fn peak_index_in_mountain_array(arr: Vec<i32>) -> i32 {
    let mut index = 0;
    for i in 0..arr.len() {
        index = i as i32;
        for j in i..arr.len() {
            if arr[i] < arr[j] {
                break;
            }
            if j == arr.len() - 1 {
                return index;
            }
        }
    }
    return index;
}

#[test]
pub fn t1() {
    assert_eq!(peak_index_in_mountain_array(vec![0, 1, 0]), 1);
}
