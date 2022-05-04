// https://leetcode.com/problems/longest-palindromic-substring/
// Given a string s, return the longest palindromic substring in s.

// Example 1:

// Input: s = "babad"
// Output: "bab"
// Explanation: "aba" is also a valid answer.
// Example 2:

// Input: s = "cbbd"
// Output: "bb"

use std::ops::Range;

pub fn longest_palindrome(s: String) -> String {
    if s.len() <= 1 {
        return s
    }

    fn palindrome(s: &str, left: usize, right: usize) -> Range<usize> {
        let mut l = left as i32;
        let mut r = right;
        // don't use s.chars().nth, it will timeout
        // https://www.reddit.com/r/rust/comments/tbsffu/why_charsnthindexunwrap_is_so_slow/
        let str_vec: Vec<char> = s.chars().collect();
        while l >= 0 && r < s.len() && str_vec[l as usize] == str_vec[r] {
            l -= 1;
            r += 1;
        }
        return (l + 1) as usize..r;
    }

    let mut longest_palindrome = 0..0;
    for i in 0..s.len() {
        // s[i] centered longest palindrome (babab -> bab)
        let s1 = palindrome(&s, i, i);
        // s[i], s[i+1] centered longest palindrome (cbbd -> bb)0
        let s2 = palindrome(&s, i, i + 1);
        longest_palindrome = [s1, s2, longest_palindrome.clone()].iter().max_by_key(|x| x.end - x.start).unwrap().clone();
    }

    return s[longest_palindrome.start..longest_palindrome.end].to_string();
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test() {
        assert_eq!(longest_palindrome(String::from("a")), "a");
        assert_eq!(longest_palindrome(String::from("bb")), "bb");
        assert_eq!(longest_palindrome(String::from("cbbd")), "bb");
        assert_eq!(longest_palindrome(String::from("babad")), "bab");
    }

    #[test]
    pub fn long_string_in_array() {
        let items = vec!["db".to_string(), "abc".to_string(), "a".to_string()];
        let max = items.into_iter().max_by_key(|i| i.len()).unwrap();
        assert_eq!(max, "abc");
    }
}
