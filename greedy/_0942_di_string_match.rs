// https://leetcode.com/problems/di-string-match
//
// A permutation `perm` of `n + 1` integers of all the integers in the range `[0, n]` can be represented as a string `s` of length `n` where:
//
// *   `s[i] == 'I'` if `perm[i] < perm[i + 1]`, and
// *   `s[i] == 'D'` if `perm[i] > perm[i + 1]`.
//
// Given a string `s`, reconstruct the permutation `perm` and return it. If there are multiple valid permutations perm, return **any of them**.
//
// **Example 1:**
//
// ```
// **Input:** s = "IDID"
// **Output:** [0,4,1,3,2]
// ```
//
// **Example 2:**
//
// ```
// **Input:** s = "III"
// **Output:** [0,1,2,3]
// ```
//
// **Example 3:**
//
// ```
// **Input:** s = "DDI"
// **Output:** [3,2,0,1]
// ```
//
// **Constraints:**
//
// *   `1 <= s.length <= 10<sup>5</sup>`
// *   `s[i]` is either `'I'` or `'D'`.

pub fn di_string_match(s: String) -> Vec<i32> {
    let mut res = vec![0 as i32; s.len() + 1];
    let mut l = 0;
    let mut h = s.len() as i32;
    for (i, c) in s.chars().enumerate() {
        if c == 'I' {
            res[i] = l;
            l += 1;
        } else {
            res[i] = h;
            h -= 1;
        }
    }
    res[s.len()] = l;
    return res;
}

#[test]
pub fn t1() {
    assert_eq!(di_string_match("IDID".to_string()), vec![0, 4, 1, 3, 2]);
    assert_eq!(di_string_match("III".to_string()), vec![0, 1, 2, 3]);
}
