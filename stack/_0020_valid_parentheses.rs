// https://leetcode.com/problems/valid-parentheses
//
// Given a string `s` containing just the characters `'('`, `')'`, `'{'`, `'}'`, `'['` and `']'`, determine if the input string is valid.
//
// An input string is valid if:
//
// 1.  Open brackets must be closed by the same type of brackets.
// 2.  Open brackets must be closed in the correct order.
//
// **Example 1:**
//
// ```
// **Input:** s = "()"
// **Output:** true
// ```
//
// **Example 2:**
//
// ```
// **Input:** s = "()[]{}"
// **Output:** true
// ```
//
// **Example 3:**
//
// ```
// **Input:** s = "(]"
// **Output:** false
// ```
//
// **Constraints:**
//
// *   `1 <= s.length <= 10<sup>4</sup>`
// *   `s` consists of parentheses only `'()[]{}'`.

pub fn is_valid(s: String) -> bool {
    let mut stack = Vec::new();
    for c in s.chars() {
        match c {
            '(' => stack.push('('),
            '{' => stack.push('{'),
            '[' => stack.push('['),
            ')' => {
                if stack.pop() != Some('(') {
                    return false;
                }
            }
            '}' => {
                if stack.pop() != Some('{') {
                    return false;
                }
            }
            ']' => {
                if stack.pop() != Some('[') {
                    return false;
                }
            }
            _ => {}
        }
    }
    return stack.is_empty();
}

#[test]
pub fn t1() {
    assert_eq!(is_valid("()".to_string()), true);
}
