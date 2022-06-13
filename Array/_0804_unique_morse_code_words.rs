// https://leetcode.com/problems/unique-morse-code-words
//
// International Morse Code defines a standard encoding where each letter is mapped to a series of dots and dashes, as follows:
//
// *   `'a'` maps to `".-"`,
// *   `'b'` maps to `"-..."`,
// *   `'c'` maps to `"-.-."`, and so on.
//
// For convenience, the full table for the `26` letters of the English alphabet is given below:
//
// ```
// [".-","-...","-.-.","-..",".","..-.","--.","....","..",".---","-.-",".-..","--","-.","---",".--.","--.-",".-.","...","-","..-","...-",".--","-..-","-.--","--.."]```
//
// Given an array of strings `words` where each word can be written as a concatenation of the Morse code of each letter.
//
// *   For example, `"cab"` can be written as `"-.-..--..."`, which is the concatenation of `"-.-."`, `".-"`, and `"-..."`. We will call such a concatenation the **transformation** of a word.
//
// Return _the number of different **transformations** among all words we have_.
//
// **Example 1:**
//
// ```
// **Input:** words = ["gin","zen","gig","msg"]
// **Output:** 2
// **Explanation:** The transformation of each word is:
// "gin" -> "--...-."
// "zen" -> "--...-."
// "gig" -> "--...--."
// "msg" -> "--...--."
// There are 2 different transformations: "--...-." and "--...--.".
// ```
//
// **Example 2:**
//
// ```
// **Input:** words = ["a"]
// **Output:** 1
// ```
//
// **Constraints:**
//
// *   `1 <= words.length <= 100`
// *   `1 <= words[i].length <= 12`
// *   `words[i]` consists of lowercase English letters.

pub fn unique_morse_representations(words: Vec<String>) -> i32 {
    let morse_code = vec![
        ".-", "-...", "-.-.", "-..", ".", "..-.", "--.", "....", "..", ".---", "-.-", ".-..", "--",
        "-.", "---", ".--.", "--.-", ".-.", "...", "-", "..-", "...-", ".--", "-..-", "-.--",
        "--..",
    ];
    words
        .iter()
        .fold(std::collections::HashSet::new(), |mut acc, word| {
            let mut word_morse = String::new();
            for c in word.chars() {
                word_morse.push_str(&morse_code[(c as u8 - 'a' as u8) as usize]);
            }
            acc.insert(word_morse);
            acc
        })
        .len() as i32
}

#[test]
pub fn t1() {
    assert_eq!(
        unique_morse_representations(vec![
            "gin".to_string(),
            "zen".to_string(),
            "gig".to_string(),
            "msg".to_string()
        ]),
        2
    );
}
