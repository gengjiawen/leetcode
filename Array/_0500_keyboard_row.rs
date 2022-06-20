// https://leetcode.com/problems/keyboard-row
//
// Given an array of strings `words`, return _the words that can be typed using letters of the alphabet on only one row of American keyboard like the image below_.
//
// In the **American keyboard**:
//
// *   the first row consists of the characters `"qwertyuiop"`,
// *   the second row consists of the characters `"asdfghjkl"`, and
// *   the third row consists of the characters `"zxcvbnm"`.
//
// ![](https://assets.leetcode.com/uploads/2018/10/12/keyboard.png)
//
// **Example 1:**
//
// ```
// **Input:** words = ["Hello","Alaska","Dad","Peace"]
// **Output:** ["Alaska","Dad"]
// ```
//
// **Example 2:**
//
// ```
// **Input:** words = ["omk"]
// **Output:** []
// ```
//
// **Example 3:**
//
// ```
// **Input:** words = ["adsdf","sfd"]
// **Output:** ["adsdf","sfd"]
// ```
//
// **Constraints:**
//
// *   `1 <= words.length <= 20`
// *   `1 <= words[i].length <= 100`
// *   `words[i]` consists of English letters (both lowercase and uppercase).

pub fn find_words(words: Vec<String>) -> Vec<String> {
    let keyboard = vec![
        "qwertyuiop".to_string(),
        "asdfghjkl".to_string(),
        "zxcvbnm".to_string(),
    ];
    return words
        .iter()
        .filter(|&w| {
            keyboard.iter().any(|row| {
                w.chars()
                    .all(|c| row.contains(c.to_lowercase().to_string().as_str()))
            })
        })
        .cloned()
        .collect();
}

#[test]
pub fn t1() {
    assert_eq!(
        find_words(vec![
            "Hello".to_string(),
            "Alaska".to_string(),
            "Dad".to_string(),
            "Peace".to_string(),
        ]),
        vec!["Alaska", "Dad"]
    );
}
