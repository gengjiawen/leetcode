// https://leetcode.com/problems/combination-sum
//
// Given an array of **distinct** integers `candidates` and a target integer `target`, return _a list of all **unique combinations** of_ `candidates` _where the chosen numbers sum to_ `target`_._ You may return the combinations in **any order**.
//
// The **same** number may be chosen from `candidates` an **unlimited number of times**. Two combinations are unique if the frequency of at least one of the chosen numbers is different.
//
// It is **guaranteed** that the number of unique combinations that sum up to `target` is less than `150` combinations for the given input.
//
// **Example 1:**
//
// ```
// **Input:** candidates = [2,3,6,7], target = 7
// **Output:** [[2,2,3],[7]]
// **Explanation:**
// 2 and 3 are candidates, and 2 + 2 + 3 = 7\. Note that 2 can be used multiple times.
// 7 is a candidate, and 7 = 7.
// These are the only two combinations.
// ```
//
// **Example 2:**
//
// ```
// **Input:** candidates = [2,3,5], target = 8
// **Output:** [[2,2,2,2],[2,3,3],[3,5]]
// ```
//
// **Example 3:**
//
// ```
// **Input:** candidates = [2], target = 1
// **Output:** []
// ```
//
// **Constraints:**
//
// *   `1 <= candidates.length <= 30`
// *   `1 <= candidates[i] <= 200`
// *   All elements of `candidates` are **distinct**.
// *   `1 <= target <= 500`

pub fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
    fn backtrace(
        res: &mut Vec<Vec<i32>>,
        cur: &mut Vec<i32>,
        candidates: &Vec<i32>,
        target: i32,
        start: usize,
    ) {
        if start == candidates.len() {
            return;
        }
        if target == 0 {
            res.push(cur.clone());
            return;
        }

        // skip it
        backtrace(res, cur, candidates, target, start + 1);

        // include it once again
        if target < candidates[start] {
            return;
        }
        cur.push(candidates[start]);
        backtrace(res, cur, candidates, target - candidates[start], start);
        cur.pop();
    }
    let mut res = Vec::new();
    backtrace(&mut res, &mut vec![], &candidates, target, 0);
    return res;
}

#[test]
pub fn t1() {
    assert_eq!(
        combination_sum(vec![2, 3, 6, 7], 7)
            .iter()
            .cloned()
            .collect::<std::collections::HashSet<Vec<i32>>>(),
        vec![vec![2, 2, 3], vec![7]]
            .iter()
            .cloned()
            .collect::<std::collections::HashSet<Vec<i32>>>(),
    )
}
