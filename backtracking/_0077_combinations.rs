// https://leetcode.com/problems/combinations
//
// Given two integers `n` and `k`, return _all possible combinations of_ `k` _numbers out of the range_ `[1, n]`.
//
// You may return the answer in **any order**.
//
// **Example 1:**
//
// ```
// **Input:** n = 4, k = 2
// **Output:**
// [
//   [2,4],
//   [3,4],
//   [2,3],
//   [1,2],
//   [1,3],
//   [1,4],
// ]
// ```
//
// **Example 2:**
//
// ```
// **Input:** n = 1, k = 1
// **Output:** [[1]]
// ```
//
// **Constraints:**
//
// *   `1 <= n <= 20`
// *   `1 <= k <= n`

pub fn combine(n: i32, k: i32) -> Vec<Vec<i32>> {
    let mut res = Vec::new();
    fn backtrack(n: i32, k: i32, cur: &mut Vec<i32>, res: &mut Vec<Vec<i32>>, start: i32) {
        if k == cur.len() as i32 {
            res.push(cur.clone());
            return;
        }
        if start > n {
            return;
        }

        // use current location
        cur.push(start);
        backtrack(n, k, cur, res, start + 1);
        cur.pop();

        // skip current location
        backtrack(n, k, cur, res, start + 1);
    }

    backtrack(n, k, &mut Vec::new(), &mut res, 1);
    return res;
}

#[test]
pub fn t1() {
    assert_eq!(
        combine(4, 2)
            .iter()
            .cloned()
            .collect::<std::collections::HashSet<Vec<i32>>>(),
        vec![
            vec![2, 4],
            vec![3, 4],
            vec![2, 3],
            vec![1, 2],
            vec![1, 3],
            vec![1, 4]
        ]
        .iter()
        .cloned()
        .collect::<std::collections::HashSet<Vec<i32>>>()
    );
}
