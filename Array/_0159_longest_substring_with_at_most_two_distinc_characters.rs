// https://leetcode.com/problems/longest-substring-with-at-most-two-distinct-characters/
fn length_of_longest_substring_two_distinct(s: String) -> i32 {
    let str: Vec<char> = s.chars().collect();
    let mut start = 0;
    let mut max = 0 as i32;
    // store char rightmost index
    let mut map = std::collections::HashMap::new();
    for end in 0..str.len() {
        if map.len() < 3 {
            map.insert(str[end], end);
        }
        if map.len() == 3 {
            // delete the leftmost character
            let del_index = *map.iter().min_by_key(|&(_, v)| v).unwrap().1;
            map.remove(&str[del_index]);
            start = del_index + 1;
        }

        max = std::cmp::max(max, (end - start + 1) as i32);
    }

    return max
}

#[test]
fn test() {
    assert_eq!(length_of_longest_substring_two_distinct("eceba".to_string()), 3);
    assert_eq!(length_of_longest_substring_two_distinct("ccaabbb".to_string()), 5);
}
