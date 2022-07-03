// https://leetcode.com/problems/find-words-that-can-be-formed-by-characters
//
// You are given an array of strings `words` and a string `chars`.
//
// A string is **good** if it can be formed by characters from chars (each character can only be used once).
//
// Return _the sum of lengths of all good strings in words_.
//
// **Example 1:**
//
// ```
// **Input:** words = ["cat","bt","hat","tree"], chars = "atach"
// **Output:** 6
// **Explanation:** The strings that can be formed are "cat" and "hat" so the answer is 3 + 3 = 6.
// ```
//
// **Example 2:**
//
// ```
// **Input:** words = ["hello","world","leetcode"], chars = "welldonehoneyr"
// **Output:** 10
// **Explanation:** The strings that can be formed are "hello" and "world" so the answer is 5 + 5 = 10.
// ```
//
// **Constraints:**
//
// *   `1 <= words.length <= 1000`
// *   `1 <= words[i].length, chars.length <= 100`
// *   `words[i]` and `chars` consist of lowercase English letters.

pub fn count_characters(words: Vec<String>, chars: String) -> i32 {
    fn word_map(s: &str) -> std::collections::HashMap<char, i32> {
        let mut map = std::collections::HashMap::new();
        for c in s.chars() {
            *map.entry(c).or_insert(0) += 1;
        }
        return map;
    }
    let chars_map = word_map(&chars);
    return words
        .iter()
        .map(|w| {
            let w_map = word_map(w);
            w_map
                .iter()
                .all(|(char, num)| chars_map.get(char).unwrap_or(&0) >= num)
                .then(|| w.len() as i32)
                .unwrap_or(0)
        })
        .sum::<i32>();
}

#[test]
pub fn t1() {
    assert_eq!(
        count_characters(
            vec![
                "cat".to_string(),
                "bt".to_string(),
                "hat".to_string(),
                "tree".to_string()
            ],
            "atach".to_string()
        ),
        6
    );
}
