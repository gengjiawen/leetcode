// https://leetcode.com/problems/count-prefixes-of-a-given-string
//
// You are given a string array `words` and a string `s`, where `words[i]` and `s` comprise only of **lowercase English letters**.
//
// Return _the **number of strings** in_ `words` _that are a **prefix** of_ `s`.
//
// A **prefix** of a string is a substring that occurs at the beginning of the string. A **substring** is a contiguous sequence of characters within a string.
//
// **Example 1:**
//
// ```
// **Input:** words = ["a","b","c","ab","bc","abc"], s = "abc"
// **Output:** 3
// **Explanation:**
// The strings in words which are a prefix of s = "abc" are:
// "a", "ab", and "abc".
// Thus the number of strings in words which are a prefix of s is 3.```
//
// **Example 2:**
//
// ```
// **Input:** words = ["a","a"], s = "aa"
// **Output:** 2
// **Explanation:** Both of the strings are a prefix of s.
// Note that the same string can occur multiple times in words, and it should be counted each time.```
//
// **Constraints:**
//
// *   `1 <= words.length <= 1000`
// *   `1 <= words[i].length, s.length <= 10`
// *   `words[i]` and `s` consist of lowercase English letters **only**.

pub fn count_prefixes(words: Vec<String>, s: String) -> i32 {
    return words.iter().filter(|&w| s.starts_with(w)).count() as i32;
}

#[test]
pub fn t1() {
    assert_eq!(
        count_prefixes(
            vec![
                "a".to_string(),
                "b".to_string(),
                "c".to_string(),
                "ab".to_string(),
                "bc".to_string(),
                "abc".to_string()
            ],
            "abc".to_string()
        ),
        3
    );
}
