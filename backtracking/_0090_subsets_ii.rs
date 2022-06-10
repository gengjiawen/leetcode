// https://leetcode.com/problems/subsets-ii
//
// Given an integer array `nums` that may contain duplicates, return _all possible subsets (the power set)_.
//
// The solution set **must not** contain duplicate subsets. Return the solution in **any order**.
//
// **Example 1:**
//
// ```
// **Input:** nums = [1,2,2]
// **Output:** [[],[1],[1,2],[1,2,2],[2],[2,2]]
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

pub fn subsets_with_dup(nums: Vec<i32>) -> Vec<Vec<i32>> {
    fn backtrack(res: &mut Vec<Vec<i32>>, nums: &Vec<i32>, cur: &mut Vec<i32>, start: usize) {
        res.push(cur.clone());
        for i in start..nums.len() {
            if i > start && nums[i] == nums[i - 1] {
                continue;
            }
            cur.push(nums[i]);
            backtrack(res, nums, cur, i + 1);
            cur.pop();
        }
    }
    let mut sorted = nums.clone();
    sorted.sort();
    let mut res = vec![];
    backtrack(&mut res, &sorted, &mut vec![], 0);
    return res;
}

#[test]
pub fn t1() {
    assert_eq!(
        subsets_with_dup(vec![1, 2, 2]),
        vec![
            vec![],
            vec![1],
            vec![1, 2],
            vec![1, 2, 2],
            vec![2],
            vec![2, 2]
        ]
    );
}
