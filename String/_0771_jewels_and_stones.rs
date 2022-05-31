// https://leetcode.com/problems/jewels-and-stones
//
// You're given strings `jewels` representing the types of stones that are jewels, and `stones` representing the stones you have. Each character in `stones` is a type of stone you have. You want to know how many of the stones you have are also jewels.
//
// Letters are case sensitive, so `"a"` is considered a different type of stone from `"A"`.
//
// **Example 1:**
//
// ```
// **Input:** jewels = "aA", stones = "aAAbbbb"
// **Output:** 3
// ```
//
// **Example 2:**
//
// ```
// **Input:** jewels = "z", stones = "ZZ"
// **Output:** 0
// ```
//
// **Constraints:**
//
// *   `1 <= jewels.length, stones.length <= 50`
// *   `jewels` and `stones` consist of only English letters.
// *   All the characters of `jewels` are **unique**.

pub fn num_jewels_in_stones(jewels: String, stones: String) -> i32 {
    let mut count = 0;
    for j in jewels.chars() {
        for s in stones.chars() {
            if j == s {
                count += 1;
            }
        }
    }
    return count;
}

pub fn num_jewels_in_stones_fast(jewels: String, stones: String) -> i32 {
    let jewels_set = jewels.chars().collect::<std::collections::HashSet<char>>();
    return stones.chars().filter(|&s| jewels_set.contains(&s)).count() as i32;
}

#[test]
pub fn t1() {
    assert_eq!(
        num_jewels_in_stones("aA".to_string(), "aAAbbbb".to_string()),
        3
    );
    assert_eq!(
        num_jewels_in_stones_fast("aA".to_string(), "aAAbbbb".to_string()),
        3
    );
}
