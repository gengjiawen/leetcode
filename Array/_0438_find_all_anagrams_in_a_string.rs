// https://leetcode.com/problems/find-all-anagrams-in-a-string
//
// Given two strings `s` and `p`, return _an array of all the start indices of_ `p`_'s anagrams in_ `s`. You may return the answer in **any order**.
//
// An **Anagram** is a word or phrase formed by rearranging the letters of a different word or phrase, typically using all the original letters exactly once.
//
// **Example 1:**
//
// ```
// **Input:** s = "cbaebabacd", p = "abc"
// **Output:** [0,6]
// **Explanation:**
// The substring with start index = 0 is "cba", which is an anagram of "abc".
// The substring with start index = 6 is "bac", which is an anagram of "abc".
// ```
//
// **Example 2:**
//
// ```
// **Input:** s = "abab", p = "ab"
// **Output:** [0,1,2]
// **Explanation:**
// The substring with start index = 0 is "ab", which is an anagram of "ab".
// The substring with start index = 1 is "ba", which is an anagram of "ab".
// The substring with start index = 2 is "ab", which is an anagram of "ab".
// ```
//
// **Constraints:**
//
// *   `1 <= s.length, p.length <= 3 * 10<sup>4</sup>`
// *   `s` and `p` consist of lowercase English letters.

pub fn find_anagrams(s: String, p: String) -> Vec<i32> {
    let mut res: Vec<i32> = vec![];
    let mut start = 0;

    let mut s_hash = std::collections::HashMap::new();
    let mut p_hash = std::collections::HashMap::new();

    let mut s_vec: Vec<char> = s.chars().collect();
    for ch in p.chars() {
        *p_hash.entry(ch).or_insert(0) += 1;
    }

    for end in 0..s.len() {
        *s_hash.entry(s_vec[end]).or_insert(0) += 1;
        if s_hash == p_hash {
            res.push(start as i32);
        }

        if end >= p.len() - 1 {
            let head = s_vec[start];
            *s_hash.entry(head).or_insert(0) -= 1;
            if s_hash[&head] == 0 {
                s_hash.remove(&head);
            }

            start += 1;
        }
    }

    return res;
}

#[test]
pub fn test() {
    assert_eq!(find_anagrams("cbaebabacd".to_string(), "abc".to_string()), vec![0, 6]);
    assert_eq!(find_anagrams("abab".to_string(), "ab".to_string()), vec![0, 1, 2]);
}
