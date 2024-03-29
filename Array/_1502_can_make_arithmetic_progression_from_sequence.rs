// https://leetcode.com/problems/can-make-arithmetic-progression-from-sequence
//
// A sequence of numbers is called an **arithmetic progression** if the difference between any two consecutive elements is the same.
//
// Given an array of numbers `arr`, return `true` _if the array can be rearranged to form an **arithmetic progression**. Otherwise, return_ `false`.
//
// **Example 1:**
//
// ```
// **Input:** arr = [3,5,1]
// **Output:** true
// **Explanation:** We can reorder the elements as [1,3,5] or [5,3,1] with differences 2 and -2 respectively, between each consecutive elements.
// ```
//
// **Example 2:**
//
// ```
// **Input:** arr = [1,2,4]
// **Output:** false
// **Explanation:** There is no way to reorder the elements to obtain an arithmetic progression.
// ```
//
// **Constraints:**
//
// *   `2 <= arr.length <= 1000`
// *   `-10<sup>6</sup> <= arr[i] <= 10<sup>6</sup>`

pub fn can_make_arithmetic_progression(arr: Vec<i32>) -> bool {
    let mut sorted_arr = arr;
    sorted_arr.sort_unstable();
    let diff = sorted_arr[1] - sorted_arr[0];
    for i in 2..sorted_arr.len() {
        if sorted_arr[i] - sorted_arr[i - 1] != diff {
            return false;
        }
    }
    return true;
}

#[test]
pub fn t1() {
    assert_eq!(can_make_arithmetic_progression(vec![3, 5, 1]), true);
}
