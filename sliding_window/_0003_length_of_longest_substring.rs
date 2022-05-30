// https://leetcode.com/problems/longest-substring-without-repeating-characters/
// Input: s = "abcabcbb"
// Output: 3
// Explanation: The answer is "abc", with the length of 3.

pub fn length_of_longest_substring(s: String) -> i32 {
    let str: Vec<char> = s.chars().collect();
    let mut start = 0;
    let mut max = 0;
    let mut map = std::collections::HashMap::new();
    for end in 0..str.len() {
        if map.contains_key(&str[end]) {
            let del_index = *map.get_key_value(&str[end]).unwrap().1;
            // can't go back to del_index + 1 like `abba`
            start = std::cmp::max(start, del_index + 1);
        }
        max = std::cmp::max(max, (end - start + 1) as i32);

        map.insert(str[end], end);
    }

    return max;
}

#[test]
pub fn test() {
    assert_eq!(length_of_longest_substring("abba".to_string()), 2);
    assert_eq!(length_of_longest_substring("abcabcbb".to_string()), 3);
    assert_eq!(length_of_longest_substring("pwwkew".to_string()), 3);
}
