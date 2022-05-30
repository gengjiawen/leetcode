// https://leetcode.com/problems/minimum-window-substring
//
// Given two strings `s` and `t` of lengths `m` and `n` respectively,
// return _the **minimum window substring** of_ `s` _such that
// every character in_ `t` _(**including duplicates**) is included in the window.
// If there is no such substring__, return the empty string_ `""`_._
//
// The testcases will be generated such that the answer is **unique**.
//
// A **substring** is a contiguous sequence of characters within the string.
//
// **Example 1:**
//
// ```
// **Input:** s = "ADOBECODEBANC", t = "ABC"
// **Output:** "BANC"
// **Explanation:** The minimum window substring "BANC" includes 'A', 'B', and 'C' from string t.
// ```
//
// **Example 2:**
//
// ```
// **Input:** s = "a", t = "a"
// **Output:** "a"
// **Explanation:** The entire string s is the minimum window.
// ```
//
// **Example 3:**
//
// ```
// **Input:** s = "a", t = "aa"
// **Output:** ""
// **Explanation:** Both 'a's from t must be included in the window.
// Since the largest window of s only has one 'a', return empty string.
// ```
//
// **Constraints:**
//
// *   `m == s.length`
// *   `n == t.length`
// *   `1 <= m, n <= 10<sup>5</sup>`
// *   `s` and `t` consist of uppercase and lowercase English letters.
//
// **Follow up:** Could you find an algorithm that runs in `O(m + n)` time?

pub fn min_window(s: String, t: String) -> String {
    let mut start = 0;

    let mut s_hash = std::collections::HashMap::new();
    let mut t_hash = std::collections::HashMap::new();

    let s_vec: Vec<char> = s.chars().collect();
    for ch in t.chars() {
        *t_hash.entry(ch).or_insert(0) += 1;
    }

    fn is_valid(
        s_hash: &std::collections::HashMap<char, i32>,
        t_hash: &std::collections::HashMap<char, i32>,
    ) -> bool {
        for (&key, &value) in t_hash {
            if s_hash.get(&key).unwrap_or(&0) < &value {
                return false;
            }
        }

        return true;
    }
    let mut min_span = s.len() + 1;

    let mut res = String::new();
    for end in 0..s.len() {
        *s_hash.entry(s_vec[end]).or_insert(0) += 1;

        while end - start + 1 >= t.len() && is_valid(&s_hash, &t_hash) {
            if is_valid(&s_hash, &t_hash) {
                if end - start + 1 < min_span {
                    min_span = end - start + 1;
                    res = s[start..end + 1].to_string();
                    if min_span == t.len() {
                        return res;
                    }
                }
            }

            let head = s_vec[start];
            *s_hash.entry(head).or_insert(0) -= 1;

            start += 1;
        }
    }

    return res;
}

#[test]
fn test() {
    assert_eq!(
        min_window("ADOBECODEBANC".to_string(), "ABC".to_string()),
        "BANC"
    );
    assert_eq!(
        min_window("cabwefgewcwaefgcf".to_string(), "cae".to_string()),
        "cwae"
    );
    assert_eq!(min_window("a".to_string(), "aa".to_string()), "");
}
