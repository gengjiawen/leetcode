// https://leetcode.com/problems/number-of-lines-to-write-string
//
// You are given a string `s` of lowercase English letters and an array `widths` denoting **how many pixels wide** each lowercase English letter is. Specifically, `widths[0]` is the width of `'a'`, `widths[1]` is the width of `'b'`, and so on.
//
// You are trying to write `s` across several lines, where **each line is no longer than** `100` **pixels**. Starting at the beginning of `s`, write as many letters on the first line such that the total width does not exceed `100` pixels. Then, from where you stopped in `s`, continue writing as many letters as you can on the second line. Continue this process until you have written all of `s`.
//
// Return _an array_ `result` _of length 2 where:_
//
// *   `result[0]` _is the total number of lines._
// *   `result[1]` _is the width of the last line in pixels._
//
// **Example 1:**
//
// ```
// **Input:** widths = [10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10], s = "abcdefghijklmnopqrstuvwxyz"
// **Output:** [3,60]
// **Explanation:** You can write s as follows:
// abcdefghij  // 100 pixels wide
// klmnopqrst  // 100 pixels wide
// uvwxyz      // 60 pixels wide
// There are a total of 3 lines, and the last line is 60 pixels wide.```
//
// **Example 2:**
//
// ```
// **Input:** widths = [4,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10], s = "bbbcccdddaaa"
// **Output:** [2,4]
// **Explanation:** You can write s as follows:
// bbbcccdddaa  // 98 pixels wide
// a            // 4 pixels wide
// There are a total of 2 lines, and the last line is 4 pixels wide.```
//
// **Constraints:**
//
// *   `widths.length == 26`
// *   `2 <= widths[i] <= 10`
// *   `1 <= s.length <= 1000`
// *   `s` contains only lowercase English letters.

pub fn number_of_lines(widths: Vec<i32>, s: String) -> Vec<i32> {
    let mut lines = 1;
    let mut width = 0;
    for c in s.chars() {
        let w = widths[c as usize - 'a' as usize];
        if width + w > 100 {
            lines += 1;
            width = w;
        } else {
            width += w;
        }
    }
    return vec![lines, width];
}

#[test]
pub fn t1() {
    assert_eq!(
        number_of_lines(
            vec![
                10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10,
                10, 10, 10, 10, 10, 10
            ],
            "abcdefghijklmnopqrstuvwxyz".to_string()
        ),
        vec![3, 60]
    );
    assert_eq!(
        number_of_lines(
            vec![
                4, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10,
                10, 10, 10, 10, 10
            ],
            "bbbcccdddaaa".to_string()
        ),
        vec![2, 4]
    );
}
