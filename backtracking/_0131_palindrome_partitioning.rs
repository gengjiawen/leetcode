// https://leetcode.com/problems/palindrome-partitioning
//
// Given a string `s`, partition `s` such that every substring of the partition is a **palindrome**. Return all possible palindrome partitioning of `s`.
//
// A **palindrome** string is a string that reads the same backward as forward.
//
// **Example 1:**
//
// ```
// **Input:** s = "aab"
// **Output:** [["a","a","b"],["aa","b"]]
// ```
//
// **Example 2:**
//
// ```
// **Input:** s = "a"
// **Output:** [["a"]]
// ```
//
// **Constraints:**
//
// *   `1 <= s.length <= 16`
// *   `s` contains only lowercase English letters.

pub fn partition(s: String) -> Vec<Vec<String>> {
    fn is_palindrome(s: &str) -> bool {
        let mut i = 0;
        let mut j = s.len() - 1;
        while i < j {
            if s.as_bytes()[i] != s.as_bytes()[j] {
                return false;
            }
            i += 1;
            j -= 1;
        }
        return true;
    }
    fn backtrack(res: &mut Vec<Vec<String>>, s: &String, cur: &mut Vec<String>, start: usize) {
        if start == s.len() {
            res.push(cur.clone());
            return;
        }
        for i in start..s.len() {
            if is_palindrome(&s[start..=i]) {
                cur.push(s[start..=i].to_string());
                backtrack(res, s, cur, i + 1);
                cur.pop();
            }
        }
    }
    let mut res = vec![];
    backtrack(&mut res, &s, &mut vec![], 0);
    return res;
}

#[test]
pub fn t1() {
    assert_eq!(
        partition("aab".to_string()),
        vec![vec!["a", "a", "b"], vec!["aa", "b"]]
    );
}
