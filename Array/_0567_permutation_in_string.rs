// https://leetcode.com/problems/permutation-in-string
//
// Given two strings `s1` and `s2`, return `true` _if_ `s2` _contains a permutation of_ `s1`_, or_ `false` _otherwise_.
//
// In other words, return `true` if one of `s1`'s permutations is the substring of `s2`.
//
// **Example 1:**
//
// ```
// **Input:** s1 = "ab", s2 = "eidbaooo"
// **Output:** true
// **Explanation:** s2 contains one permutation of s1 ("ba").
// ```
//
// **Example 2:**
//
// ```
// **Input:** s1 = "ab", s2 = "eidboaoo"
// **Output:** false
// ```
//
// **Constraints:**
//
// *   `1 <= s1.length, s2.length <= 10<sup>4</sup>`
// *   `s1` and `s2` consist of lowercase English letters.

pub fn check_inclusion(s1: String, s2: String) -> bool {
    let mut start = 0;

    let mut s1_hash = std::collections::HashMap::new();
    let mut s2_hash = std::collections::HashMap::new();

    let mut s2_vec: Vec<char> = s2.chars().collect();
    for ch in s1.chars() {
        *s1_hash.entry(ch).or_insert(0) += 1;
    }

    for end in 0..s2.len() {
        *s2_hash.entry(s2_vec[end]).or_insert(0) += 1;
        if s1_hash == s2_hash {
            return true;
        }

        if end >= s1.len() - 1 {
            let head = s2_vec[start];
            *s2_hash.entry(head).or_insert(0) -= 1;
            if s2_hash[&head] == 0 {
                s2_hash.remove(&head);
            }

            start += 1;
        }
    }

    return false;
}

#[test]
pub fn test() {
    assert_eq!(check_inclusion("ab".to_string(), "eidbaooo".to_string()), true);
}
