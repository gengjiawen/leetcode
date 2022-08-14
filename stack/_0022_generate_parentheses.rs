// https://leetcode.com/problems/generate-parentheses
//
// Given `n` pairs of parentheses, write a function to _generate all combinations of well-formed parentheses_.
//
// **Example 1:**
//
// ```
// **Input:** n = 3
// **Output:** ["((()))","(()())","(())()","()(())","()()()"]
// ```
//
// **Example 2:**
//
// ```
// **Input:** n = 1
// **Output:** ["()"]
// ```
//
// **Constraints:**
//
// *   `1 <= n <= 8`

pub fn generate_parenthesis(n: i32) -> Vec<String> {
    let mut res = vec![];
    fn backtrack(
        n: i32,
        open_count: i32,
        close_count: i32,
        cur: &mut String,
        res: &mut Vec<String>,
    ) {
        if open_count == n && close_count == n {
            res.push(cur.clone());
            return;
        }
        if open_count < n {
            cur.push('(');
            backtrack(n, open_count + 1, close_count, cur, res);
            cur.pop();
        }
        if close_count < open_count {
            cur.push(')');
            backtrack(n, open_count, close_count + 1, cur, res);
            cur.pop();
        }
    }

    backtrack(n, 0, 0, &mut String::new(), &mut res);
    return res;
}

#[test]
pub fn t1() {
    assert_eq!(
        generate_parenthesis(3),
        vec![
            "((()))".to_string(),
            "(()())".to_string(),
            "(())()".to_string(),
            "()(())".to_string(),
            "()()()".to_string(),
        ]
    );
}
