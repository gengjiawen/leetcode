// https://leetcode.com/problems/kth-distinct-string-in-an-array
//
// A **distinct string** is a string that is present only **once** in an array.
//
// Given an array of strings `arr`, and an integer `k`, return _the_ `k<sup>th</sup>` _**distinct string** present in_ `arr`. If there are **fewer** than `k` distinct strings, return _an **empty string**_ `""`.
//
// Note that the strings are considered in the **order in which they appear** in the array.
//
// **Example 1:**
//
// ```
// **Input:** arr = ["d","b","c","b","c","a"], k = 2
// **Output:** "a"
// **Explanation:**
// The only distinct strings in arr are "d" and "a".
// "d" appears 1<sup>st</sup>, so it is the 1<sup>st</sup> distinct string.
// "a" appears 2<sup>nd</sup>, so it is the 2<sup>nd</sup> distinct string.
// Since k == 2, "a" is returned.
// ```
//
// **Example 2:**
//
// ```
// **Input:** arr = ["aaa","aa","a"], k = 1
// **Output:** "aaa"
// **Explanation:**
// All strings in arr are distinct, so the 1<sup>st</sup> string "aaa" is returned.
// ```
//
// **Example 3:**
//
// ```
// **Input:** arr = ["a","b","a"], k = 3
// **Output:** ""
// **Explanation:**
// The only distinct string is "b". Since there are fewer than 3 distinct strings, we return an empty string "".
// ```
//
// **Constraints:**
//
// *   `1 <= k <= arr.length <= 1000`
// *   `1 <= arr[i].length <= 5`
// *   `arr[i]` consists of lowercase English letters.

pub fn kth_distinct(arr: Vec<String>, k: i32) -> String {
    let map = arr
        .iter()
        .fold(std::collections::HashMap::new(), |mut acc, x| {
            *acc.entry(x).or_insert(0) += 1;
            acc
        })
        .into_iter()
        .filter(|&(_k, v)| v == 1)
        .collect::<std::collections::HashMap<&String, i32>>();
    let mut count = 0;
    for s in arr.iter() {
        if map.get(s).is_some() {
            count += 1;
            if k == count {
                return s.to_string();
            }
        }
    }
    return "".to_string();
}

#[test]
pub fn t1() {
    assert_eq!(
        kth_distinct(
            vec!["d", "b", "c", "b", "c", "a"]
                .iter()
                .map(|&i| i.to_string())
                .collect(),
            2
        ),
        "a"
    );
}
