// https://leetcode.com/problems/longest-palindromic-substring/
// Given a string s, return the longest palindromic substring in s.

// Example 1:

// Input: s = "babad"
// Output: "bab"
// Explanation: "aba" is also a valid answer.
// Example 2:

// Input: s = "cbbd"
// Output: "bb"

// elegant solution but timeout :(
pub fn longest_palindrome(s: String) -> String {
    if s.len() <=1 {
        return s
    }
    let mut longest_palindrome = String::new();
    for i in 0..s.len() {
        // s[i] centered longest palindrome (babab -> bab)
        let s1 = palindrome(s.clone(), i, i);
        // s[i], s[i+1] centered longest palindrome (cbbd -> bb)0
        let s2 = palindrome(s.clone(), i, i + 1);
        longest_palindrome = [s1, s2, longest_palindrome.clone()].iter().max_by_key(|x| x.len()).unwrap().to_string();
    }

    return longest_palindrome;
}


pub fn palindrome(s: String, left: usize, right: usize) -> String {
    let mut l = left as i32;
    let mut r = right; // sorry, I am rust
    while l >= 0 && r < s.len() && s.chars().nth(l as usize) == s.chars().nth(r) {
        l -= 1;
        r += 1;
    }
    return s[(l + 1) as usize..r].to_string();
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
