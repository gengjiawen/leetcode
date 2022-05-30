// https://leetcode.com/problems/single-row-keyboard

pub fn calculate_time(keyboard: String, word: String) -> i32 {
    let mut keyboard_map = std::collections::HashMap::new();
    keyboard.chars().enumerate().for_each(|(i, c)| {
        keyboard_map.insert(c, i as i32);
    });
    let mut final_len = 0;
    let mut pre_index = 0;
    for c in word.chars() {
        final_len = final_len + i32::abs(keyboard_map[&c] - pre_index);
        pre_index = keyboard_map[&c];
    }
    return final_len;
}

#[test]
pub fn t1() {
    assert_eq!(
        calculate_time("abcdefghijklmnopqrstuvwxyz".to_string(), "cba".to_string()),
        4
    );
    assert_eq!(
        calculate_time(
            "pqrstuvwxyzabcdefghijklmno".to_string(),
            "leetcode".to_string()
        ),
        73
    );
}
