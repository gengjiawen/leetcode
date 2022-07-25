// https://leetcode.com/problems/duplicate-zeros
//
// Given a fixed-length integer array `arr`, duplicate each occurrence of zero,
// shifting the remaining elements to the right.
//
// **Note** that elements beyond the length of the original array are not written.
// Do the above modifications to the input array in place and do not return anything.
//
// **Example 1:**
//
// ```
// **Input:** arr = [1,0,2,3,0,4,5,0]
// **Output:** [1,0,0,2,3,0,0,4]
// **Explanation:** After calling your function, the input array is modified to: [1,0,0,2,3,0,0,4]
// ```
//
// **Example 2:**
//
// ```
// **Input:** arr = [1,2,3]
// **Output:** [1,2,3]
// **Explanation:** After calling your function, the input array is modified to: [1,2,3]
// ```
//
// **Constraints:**
//
// *   `1 <= arr.length <= 10<sup>4</sup>`
// *   `0 <= arr[i] <= 9`

pub fn duplicate_zeros(arr: &mut Vec<i32>) {
    let mut new_arr = Vec::with_capacity(arr.len());
    let mut new_arr_index = 0;
    for i in 0..arr.len() {
        if arr[i] == 0 {
            new_arr.insert(new_arr_index, 0);
            new_arr_index += 1;
            if new_arr_index >= arr.len() {
                break;
            }
            new_arr.insert(new_arr_index, 0);
            new_arr_index += 1;
        } else {
            new_arr.insert(new_arr_index, arr[i]);
            new_arr_index += 1;
        }
        if new_arr_index >= arr.len() {
            break;
        }
    }
    *arr = new_arr;
}

#[test]
pub fn t1() {
    let mut arr = vec![1, 0, 2, 3, 0, 4, 5, 0];
    duplicate_zeros(&mut arr);
    assert_eq!(arr, vec![1, 0, 0, 2, 3, 0, 0, 4]);

    let mut arr = vec![0, 0, 0, 0, 0, 0, 0];
    duplicate_zeros(&mut arr);
    assert_eq!(arr, vec![0, 0, 0, 0, 0, 0, 0]);
}
