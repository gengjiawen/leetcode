// https://leetcode.com/problems/subsets
//
// Given an integer array `nums` of **unique** elements, return _all possible subsets (the power set)_.
//
// The solution set **must not** contain duplicate subsets. Return the solution in **any order**.
//
// **Example 1:**
//
// ```
// **Input:** nums = [1,2,3]
// **Output:** [[],[1],[2],[1,2],[3],[1,3],[2,3],[1,2,3]]
// ```
//
// **Example 2:**
//
// ```
// **Input:** nums = [0]
// **Output:** [[],[0]]
// ```
//
// **Constraints:**
//
// *   `1 <= nums.length <= 10`
// *   `-10 <= nums[i] <= 10`
// *   All the numbers of `nums` are **unique**.

use std::collections::HashSet;
pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut res = vec![];
    let mut cur = vec![];
    fn backtrack(res: &mut Vec<Vec<i32>>, cur: &mut Vec<i32>, nums: &Vec<i32>, start: usize) {
        res.push(cur.clone());
        for i in start..nums.len() {
            cur.push(nums[i]);
            backtrack(res, cur, nums, i + 1);
            cur.pop();
        }
    }
    backtrack(&mut res, &mut cur, &nums, 0);
    return res;
}

#[test]
pub fn t1() {
    assert_eq!(
        subsets(vec![1, 2, 3])
            .iter()
            .cloned()
            .collect::<HashSet<Vec<i32>>>(),
        vec![
            vec![],
            vec![1],
            vec![2],
            vec![1, 2],
            vec![3],
            vec![1, 3],
            vec![2, 3],
            vec![1, 2, 3],
        ]
        .iter()
        .cloned()
        .collect::<HashSet<Vec<i32>>>()
    );
}
