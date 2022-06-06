// https://leetcode.com/problems/letter-combinations-of-a-phone-number
//
// Given a string containing digits from `2-9` inclusive, return all possible letter combinations that the number could represent. Return the answer in **any order**.
//
// A mapping of digit to letters (just like on the telephone buttons) is given below. Note that 1 does not map to any letters.
//
// ![](https://upload.wikimedia.org/wikipedia/commons/thumb/7/73/Telephone-keypad2.svg/200px-Telephone-keypad2.svg.png)
//
// **Example 1:**
//
// ```
// **Input:** digits = "23"
// **Output:** ["ad","ae","af","bd","be","bf","cd","ce","cf"]
// ```
//
// **Example 2:**
//
// ```
// **Input:** digits = ""
// **Output:** []
// ```
//
// **Example 3:**
//
// ```
// **Input:** digits = "2"
// **Output:** ["a","b","c"]
// ```
//
// **Constraints:**
//
// *   `0 <= digits.length <= 4`
// *   `digits[i]` is a digit in the range `['2', '9']`.



pub fn letter_combinations(digits: String) -> Vec<String> {
    if digits == "" {
        return vec![];
    }
    let digits_to_char = std::collections::HashMap::from([
        ('2', "abc"),
        ('3', "def"),
        ('4', "ghi"),
        ('5', "jkl"),
        ('6', "mno"),
        ('7', "pqrs"),
        ('8', "tuv"),
        ('9', "wxyz"),
    ]);
    struct Env<'a> {
        res: Vec<String>,
        digits: String,
        digits_to_char: std::collections::HashMap<char, &'a str>,
    }
    let mut env = Env {
        res: vec![],
        digits,
        digits_to_char,
    };
    fn backtrack(i: i32, cur_str: String, env: &mut Env) {
        if cur_str.len() == env.digits.len() {
            env.res.push(cur_str);
            return;
        }

        let cur_char = env.digits.chars().nth(i as usize).unwrap();
        for c in env.digits_to_char[&cur_char].chars() {
            backtrack(i + 1, format!("{}{}", cur_str, c), env);
        }
    }
    backtrack(0, "".to_string(), &mut env);

    return env.res;
}

#[test]
pub fn t1() {
    assert_eq!(letter_combinations("23".to_string()), vec!["ad", "ae", "af", "bd", "be", "bf", "cd", "ce", "cf"]);
}
