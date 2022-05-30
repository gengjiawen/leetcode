// https://leetcode.com/problems/get-equal-substrings-within-budget
//
// You are given two strings `s` and `t` of the same length and an integer `maxCost`.
//
// You want to change `s` to `t`. Changing the `i<sup>th</sup>` character of `s` to `i<sup>th</sup>` character of `t` costs `|s[i] - t[i]|` (i.e., the absolute difference between the ASCII values of the characters).
//
// Return _the maximum length of a substring of_ `s` _that can be changed to be the same as the corresponding substring of_ `t` _with a cost less than or equal to_ `maxCost`.
// If there is no substring from `s` that can be changed to its corresponding substring from `t`, return `0`.
//
// **Example 1:**
//
// ```
// **Input:** s = "abcd", t = "bcdf", maxCost = 3
// **Output:** 3
// **Explanation:** "abc" of s can change to "bcd".
// That costs 3, so the maximum length is 3.
// ```
//
// **Example 2:**
//
// ```
// **Input:** s = "abcd", t = "cdef", maxCost = 3
// **Output:** 1
// **Explanation:** Each character in s costs 2 to change to character in t,  so the maximum length is 1.
// ```
//
// **Example 3:**
//
// ```
// **Input:** s = "abcd", t = "acde", maxCost = 0
// **Output:** 1
// **Explanation:** You cannot make any change, so the maximum length is 1.
// ```
//
// **Constraints:**
//
// *   `1 <= s.length <= 10<sup>5</sup>`
// *   `t.length == s.length`
// *   `0 <= maxCost <= 10<sup>6</sup>`
// *   `s` and `t` consist of only lowercase English letters.

pub fn equal_substring(s: String, t: String, max_cost: i32) -> i32 {
    let mut max_len = 0;
    let mut start = 0;
    let mut cur_cost = 0;
    let s_vec = s.chars().collect::<Vec<char>>();
    let t_vec = t.chars().collect::<Vec<char>>();
    for end in 0..t.len() {
        cur_cost += (s_vec[end] as i32 - t_vec[end] as i32).abs();
        while cur_cost > max_cost {
            cur_cost -= (s_vec[start] as i32 - t_vec[start] as i32).abs();
            start += 1;
        }

        if end - start + 1 > max_len {
            max_len = end - start + 1;
        }
    }

    return max_len as i32;
}

#[test]
pub fn test() {
    assert_eq!(
        equal_substring("abcd".to_string(), "bcdf".to_string(), 3),
        3
    );
}
