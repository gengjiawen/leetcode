// https://leetcode.com/problems/evaluate-reverse-polish-notation
//
// Evaluate the value of an arithmetic expression in [Reverse Polish Notation](http://en.wikipedia.org/wiki/Reverse_Polish_notation).
//
// Valid operators are `+`, `-`, `*`, and `/`. Each operand may be an integer or another expression.
//
// **Note** that division between two integers should truncate toward zero.
//
// It is guaranteed that the given RPN expression is always valid. That means the expression would always evaluate to a result, and there will not be any division by zero operation.
//
// **Example 1:**
//
// ```
// **Input:** tokens = ["2","1","+","3","*"]
// **Output:** 9
// **Explanation:** ((2 + 1) * 3) = 9
// ```
//
// **Example 2:**
//
// ```
// **Input:** tokens = ["4","13","5","/","+"]
// **Output:** 6
// **Explanation:** (4 + (13 / 5)) = 6
// ```
//
// **Example 3:**
//
// ```
// **Input:** tokens = ["10","6","9","3","+","-11","*","/","*","17","+","5","+"]
// **Output:** 22
// **Explanation:** ((10 * (6 / ((9 + 3) * -11))) + 17) + 5
// = ((10 * (6 / (12 * -11))) + 17) + 5
// = ((10 * (6 / -132)) + 17) + 5
// = ((10 * 0) + 17) + 5
// = (0 + 17) + 5
// = 17 + 5
// = 22
// ```
//
// **Constraints:**
//
// *   `1 <= tokens.length <= 10<sup>4</sup>`
// *   `tokens[i]` is either an operator: `"+"`, `"-"`, `"*"`, or `"/"`, or an integer in the range `[-200, 200]`.

pub fn eval_rpn(tokens: Vec<String>) -> i32 {
    let mut stack = Vec::new();
    for token in tokens {
        if token == "+" {
            let a = stack.pop().unwrap();
            let b = stack.pop().unwrap();
            stack.push(a + b);
        } else if token == "-" {
            let a = stack.pop().unwrap();
            let b = stack.pop().unwrap();
            stack.push(b - a);
        } else if token == "*" {
            let a = stack.pop().unwrap();
            let b = stack.pop().unwrap();
            stack.push(a * b);
        } else if token == "/" {
            let a = stack.pop().unwrap();
            let b = stack.pop().unwrap();
            stack.push(b / a);
        } else {
            stack.push(token.parse::<i32>().unwrap());
        }
    }
    return stack.pop().unwrap();
}

#[test]
pub fn t1() {
    assert_eq!(
        eval_rpn(
            vec!["2", "1", "+", "3", "*"]
                .iter()
                .map(|x| x.to_string())
                .collect()
        ),
        9
    );
}
