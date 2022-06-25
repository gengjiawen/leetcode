// https://leetcode.com/problems/find-common-characters
//
// Given a string array `words`, return _an array of all characters that show up in all strings within the_ `words` _(including duplicates)_. You may return the answer in **any order**.
//
// **Example 1:**
//
// ```
// **Input:** words = ["bella","label","roller"]
// **Output:** ["e","l","l"]
// ```
//
// **Example 2:**
//
// ```
// **Input:** words = ["cool","lock","cook"]
// **Output:** ["c","o"]
// ```
//
// **Constraints:**
//
// *   `1 <= words.length <= 100`
// *   `1 <= words[i].length <= 100`
// *   `words[i]` consists of lowercase English letters.

pub fn common_chars(words: Vec<String>) -> Vec<String> {
    let mut word_0 = words[0]
        .chars()
        .fold(std::collections::HashMap::new(), |mut acc, x| {
            *acc.entry(x).or_insert(0) += 1;
            acc
        });
    for other_word in words[1..].iter() {
        let other_chars =
            other_word
                .chars()
                .fold(std::collections::HashMap::new(), |mut acc, x| {
                    *acc.entry(x).or_insert(0) += 1;
                    acc
                });
        for (k, v) in word_0.iter_mut() {
            *v = std::cmp::min(*v, *other_chars.get(k).unwrap_or(&0));
        }
    }
    return word_0
        .into_iter()
        .filter(|&(_k, v)| v > 0)
        .map(|(k, v)| [k].repeat(v))
        .flatten()
        .map(|x| x.to_string())
        .collect();
}

#[test]
pub fn t1() {
    assert_eq!(
        common_chars(
            vec!["bella", "label", "roller"]
                .iter()
                .map(|&i| i.to_string())
                .collect()
        ),
        vec!["e", "l", "l"]
    );
}
