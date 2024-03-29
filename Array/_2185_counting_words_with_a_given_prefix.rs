// https://leetcode.com/problems/counting-words-with-a-given-prefix
//
// You are given an array of strings `words` and a string `pref`.
//
// Return _the number of strings in_ `words` _that contain_ `pref` _as a **prefix**_.
//
// A **prefix** of a string `s` is any leading contiguous substring of `s`.
//
// **Example 1:**
//
// ```
// **Input:** words = ["pay","**<u>at</u>**tention","practice","<u>**at**</u>tend"], `pref` = "at"
// **Output:** 2
// **Explanation:** The 2 strings that contain "at" as a prefix are: "<u>**at**</u>tention" and "<u>**at**</u>tend".
// ```
//
// **Example 2:**
//
// ```
// **Input:** words = ["leetcode","win","loops","success"], `pref` = "code"
// **Output:** 0
// **Explanation:** There are no strings that contain "code" as a prefix.
// ```
//
// **Constraints:**
//
// *   `1 <= words.length <= 100`
// *   `1 <= words[i].length, pref.length <= 100`
// *   `words[i]` and `pref` consist of lowercase English letters.

pub fn prefix_count(words: Vec<String>, pref: String) -> i32 {
    return words.iter().filter(|&w| w.starts_with(&pref)).count() as i32;
}

#[test]
pub fn t1() {
    assert_eq!(
        prefix_count(
            vec![
                "pay".to_string(),
                "attention".to_string(),
                "practice".to_string(),
                "attend".to_string()
            ],
            "at".to_string()
        ),
        2
    );
}
