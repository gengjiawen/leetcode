// https://leetcode.com/problems/remove-vowels-from-a-string

pub fn remove_vowels(s: String) -> String {
    return s
        .chars()
        .filter(|i| !vec!['a', 'o', 'e', 'i', 'u'].contains(i))
        .collect();
}

#[test]
pub fn t1() {
    assert_eq!(remove_vowels("aeiou".to_string()), "".to_string());
}
