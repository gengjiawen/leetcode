// Input: s = "abcabcbb"
// Output: 3
// Explanation: The answer is "abc", with the length of 3.


pub fn length_of_longest_substring(s: String) -> i32 {
    let str = s.as_bytes();
    let mut start = 0;
    let mut max = 0 as i32;
    for mut end in 0..str.len() {
        for idx in start..end {
            if str[idx] == str[end] {
                start = idx + 1;
                break;
            }
        }
        max = std::cmp::max(max, (end - start + 1) as i32);
        end += 1;
    }

    return max

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test() {
        assert_eq!(length_of_longest_substring("abcabcbb".to_string()), 3);
    }
}
