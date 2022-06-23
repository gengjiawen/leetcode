// https://leetcode.com/problems/count-common-words-with-one-occurrence
//
// Given two string arrays `words1` and `words2`, return _the number of strings that appear **exactly once** in **each** of the two arrays._
//
// **Example 1:**
//
// ```
// **Input:** words1 = ["leetcode","is","amazing","as","is"], words2 = ["amazing","leetcode","is"]
// **Output:** 2
// **Explanation:**
// - "leetcode" appears exactly once in each of the two arrays. We count this string.
// - "amazing" appears exactly once in each of the two arrays. We count this string.
// - "is" appears in each of the two arrays, but there are 2 occurrences of it in words1\. We do not count this string.
// - "as" appears once in words1, but does not appear in words2\. We do not count this string.
// Thus, there are 2 strings that appear exactly once in each of the two arrays.
// ```
//
// **Example 2:**
//
// ```
// **Input:** words1 = ["b","bb","bbb"], words2 = ["a","aa","aaa"]
// **Output:** 0
// **Explanation:** There are no strings that appear in each of the two arrays.
// ```
//
// **Example 3:**
//
// ```
// **Input:** words1 = ["a","ab"], words2 = ["a","a","a","ab"]
// **Output:** 1
// **Explanation:** The only string that appears exactly once in each of the two arrays is "ab".
// ```
//
// **Constraints:**
//
// *   `1 <= words1.length, words2.length <= 1000`
// *   `1 <= words1[i].length, words2[j].length <= 30`
// *   `words1[i]` and `words2[j]` consists only of lowercase English letters.

pub fn count_words(words1: Vec<String>, words2: Vec<String>) -> i32 {
    let w1 = words1
        .iter()
        .fold(std::collections::HashMap::new(), |mut acc, x| {
            *acc.entry(x).or_insert(0) += 1;
            acc
        });
    let w2 = words2
        .iter()
        .fold(std::collections::HashMap::new(), |mut acc, x| {
            *acc.entry(x).or_insert(0) += 1;
            acc
        })
        .into_iter()
        .filter(|&(_k, v)| v == 1)
        .collect::<std::collections::HashMap<&String, i32>>();

    let mut count = 0;
    for (k, v) in w1.iter() {
        if *v == 1 && w2.get(k).is_some() {
            count += 1;
        }
    }
    return count;
}

#[test]
pub fn t1() {
    assert_eq!(
        count_words(
            vec![
                "leetcode".to_string(),
                "is".to_string(),
                "amazing".to_string(),
                "as".to_string(),
                "is".to_string()
            ],
            vec![
                "amazing".to_string(),
                "leetcode".to_string(),
                "is".to_string()
            ]
        ),
        2
    );
}
