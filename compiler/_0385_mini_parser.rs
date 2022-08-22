// https://leetcode.com/problems/mini-parser
//
// Given a string s represents the serialization of a nested list, implement a parser to deserialize it and return _the deserialized_ `NestedInteger`.
//
// Each element is either an integer or a list whose elements may also be integers or other lists.
//
// **Example 1:**
//
// ```
// **Input:** s = "324"
// **Output:** 324
// **Explanation:** You should return a NestedInteger object which contains a single integer 324.
// ```
//
// **Example 2:**
//
// ```
// **Input:** s = "[123,[456,[789]]]"
// **Output:** [123,[456,[789]]]
// **Explanation:** Return a NestedInteger object containing a nested list with 2 elements:
// 1\. An integer containing value 123.
// 2\. A nested list containing two elements:
//     i.  An integer containing value 456.
//     ii. A nested list with one element:
//          a. An integer containing value 789
// ```
//
// **Constraints:**
//
// *   `1 <= s.length <= 5 * 10<sup>4</sup>`
// *   `s` consists of digits, square brackets `"[]"`, negative sign `'-'`, and commas `','`.
// *   `s` is the serialization of valid `NestedInteger`.
// *   All the values in the input are in the range `[-10<sup>6</sup>, 10<sup>6</sup>]`.

#[derive(Debug, PartialEq, Eq)]
pub enum NestedInteger {
    Int(i32),
    List(Vec<NestedInteger>),
}

pub fn deserialize(s: String) -> NestedInteger {
    let chars = s.chars().collect::<Vec<char>>();
    if chars[0] != '[' {
        return NestedInteger::Int(s.parse::<i32>().unwrap());
    }
    fn is_digit(c: char) -> bool {
        c >= '0' && c <= '9'
    }
    let mut stack = Vec::new();
    let mut negative = false;
    let mut i = 0;
    while i < chars.len() {
        match chars[i] {
            '[' => {
                stack.push(NestedInteger::List(Vec::new()));
            }
            '-' => {
                negative = true;
            }
            ']' => {
                if stack.len() > 1 {
                    let top = stack.pop().unwrap();
                    if let NestedInteger::List(v) = stack.last_mut().unwrap() {
                        v.push(top)
                    }
                }
            }
            _ => {
                if is_digit(chars[i]) {
                    let mut num = String::new();
                    while is_digit(chars[i]) {
                        num.push(chars[i]);
                        i += 1;
                    }
                    let num = if negative {
                        num.parse::<i32>().unwrap() * -1
                    } else {
                        num.parse::<i32>().unwrap()
                    };
                    if let NestedInteger::List(v) = stack.last_mut().unwrap() {
                        v.push(NestedInteger::Int(num))
                    }
                    // reset negative state
                    negative = false;
                    continue;
                }
            }
        }
        i += 1;
    }
    return stack.pop().unwrap();
}

#[test]
pub fn t1() {
    assert_eq!(deserialize("324".to_string()), NestedInteger::Int(324));
    assert_eq!(
        deserialize("[123,[456,[789]]]".to_string()),
        NestedInteger::List(vec![
            NestedInteger::Int(123),
            NestedInteger::List(vec![
                NestedInteger::Int(456),
                NestedInteger::List(vec![NestedInteger::Int(789)])
            ])
        ])
    );
    assert_eq!(
        deserialize("[123,456,[788,799,833],[[]],10,[]]".to_string()),
        NestedInteger::List(vec![
            NestedInteger::Int(123),
            NestedInteger::Int(456),
            NestedInteger::List(vec![
                NestedInteger::Int(788),
                NestedInteger::Int(799),
                NestedInteger::Int(833)
            ]),
            NestedInteger::List(vec![NestedInteger::List(vec![])]),
            NestedInteger::Int(10),
            NestedInteger::List(vec![])
        ])
    );
}
