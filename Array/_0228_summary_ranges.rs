// https://leetcode.com/problems/summary-ranges
//
// You are given a **sorted unique** integer array `nums`.
//
// A **range** `[a,b]` is the set of all integers from `a` to `b` (inclusive).
//
// Return _the **smallest sorted** list of ranges that **cover all the numbers in the array exactly**_. That is, each element of `nums` is covered by exactly one of the ranges, and there is no integer `x` such that `x` is in one of the ranges but not in `nums`.
//
// Each range `[a,b]` in the list should be output as:
//
// *   `"a->b"` if `a != b`
// *   `"a"` if `a == b`
//
// **Example 1:**
//
// ```
// **Input:** nums = [0,1,2,4,5,7]
// **Output:** ["0->2","4->5","7"]
// **Explanation:** The ranges are:
// [0,2] --> "0->2"
// [4,5] --> "4->5"
// [7,7] --> "7"
// ```
//
// **Example 2:**
//
// ```
// **Input:** nums = [0,2,3,4,6,8,9]
// **Output:** ["0","2->4","6","8->9"]
// **Explanation:** The ranges are:
// [0,0] --> "0"
// [2,4] --> "2->4"
// [6,6] --> "6"
// [8,9] --> "8->9"
// ```
//
// **Constraints:**
//
// *   `0 <= nums.length <= 20`
// *   `-2<sup>31</sup> <= nums[i] <= 2<sup>31</sup> - 1`
// *   All the values of `nums` are **unique**.
// *   `nums` is sorted in ascending order.

pub fn summary_ranges(nums: Vec<i32>) -> Vec<String> {
    let mut result = Vec::new();
    if nums.len() == 0 {
        return result;
    }
    let mut start = nums[0];
    let mut end = nums[0];
    for i in 1..nums.len() {
        if nums[i] == end + 1 {
            end = nums[i];
        } else {
            if start == end {
                result.push(format!("{}", start));
            } else {
                result.push(format!("{}->{}", start, end));
            }
            start = nums[i];
            end = nums[i];
        }
    }
    if start == end {
        result.push(format!("{}", start));
    } else {
        result.push(format!("{}->{}", start, end));
    }
    return result;
}

#[test]
pub fn t1() {
    assert_eq!(
        summary_ranges(vec![0, 1, 2, 4, 5, 7]),
        vec!["0->2", "4->5", "7"]
    );
}
