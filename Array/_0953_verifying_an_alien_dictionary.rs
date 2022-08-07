// https://leetcode.com/problems/verifying-an-alien-dictionary
//
// In an alien language, surprisingly, they also use English lowercase letters, but possibly in a different `order`. The `order` of the alphabet is some permutation of lowercase letters.
//
// Given a sequence of `words` written in the alien language, and the `order` of the alphabet, return `true` if and only if the given `words` are sorted lexicographically in this alien language.
//
// **Example 1:**
//
// ```
// **Input:** words = ["hello","leetcode"], order = "hlabcdefgijkmnopqrstuvwxyz"
// **Output:** true
// **Explanation:** As 'h' comes before 'l' in this language, then the sequence is sorted.
// ```
//
// **Example 2:**
//
// ```
// **Input:** words = ["word","world","row"], order = "worldabcefghijkmnpqstuvxyz"
// **Output:** false
// **Explanation:** As 'd' comes after 'l' in this language, then words[0] > words[1], hence the sequence is unsorted.
// ```
//
// **Example 3:**
//
// ```
// **Input:** words = ["apple","app"], order = "abcdefghijklmnopqrstuvwxyz"
// **Output:** false
// **Explanation:** The first three characters "app" match, and the second string is shorter (in size.) According to lexicographical rules "apple" > "app", because 'l' > '∅', where '∅' is defined as the blank character which is less than any other character ([More info](https://en.wikipedia.org/wiki/Lexicographical_order)).
// ```
//
// **Constraints:**
//
// *   `1 <= words.length <= 100`
// *   `1 <= words[i].length <= 20`
// *   `order.length == 26`
// *   All characters in `words[i]` and `order` are English lowercase letters.

pub fn is_alien_sorted(words: Vec<String>, order: String) -> bool {
    if words.len() == 1 {
        return true;
    }
    let order_map = order
        .chars()
        .enumerate()
        .map(|(i, c)| (c, i))
        .collect::<std::collections::HashMap<char, usize>>();
    for i in 0..words.len() - 1 {
        let word1 = words[i].chars().collect::<Vec<char>>();
        let word2 = words[i + 1].chars().collect::<Vec<char>>();
        let mut j = 0;
        while j < word1.len() && j < word2.len() {
            if order_map[&word1[j]] < order_map[&word2[j]] {
                break;
            } else if order_map[&word1[j]] == order_map[&word2[j]] {
                j += 1;
            } else {
                return false;
            }
        }
        if j == word2.len() && j < word1.len() {
            return false;
        }
    }
    return true;
}

#[test]
pub fn t1() {
    assert_eq!(
        is_alien_sorted(
            vec!["hello".to_string(), "leetcode".to_string()],
            "hlabcdefgijkmnopqrstuvwxyz".to_string()
        ),
        true
    );
    assert_eq!(
        is_alien_sorted(
            vec!["apple".to_string(), "app".to_string()],
            "abcdefghijklmnopqrstuvwxyz".to_string()
        ),
        false
    );
}
