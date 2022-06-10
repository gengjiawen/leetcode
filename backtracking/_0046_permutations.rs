// https://leetcode.com/problems/permutations
//
// Given an array `nums` of distinct integers, return _all the possible permutations_. You can return the answer in **any order**.
//
// **Example 1:**
//
// ```
// **Input:** nums = [1,2,3]
// **Output:** [[1,2,3],[1,3,2],[2,1,3],[2,3,1],[3,1,2],[3,2,1]]
// ```
//
// **Example 2:**
//
// ```
// **Input:** nums = [0,1]
// **Output:** [[0,1],[1,0]]
// ```
//
// **Example 3:**
//
// ```
// **Input:** nums = [1]
// **Output:** [[1]]
// ```
//
// **Constraints:**
//
// *   `1 <= nums.length <= 6`
// *   `-10 <= nums[i] <= 10`
// *   All the integers of `nums` are **unique**.

pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
    fn backtrace(nums: &Vec<i32>, cur: &mut Vec<i32>, res: &mut Vec<Vec<i32>>) {
        if cur.len() == nums.len() {
            res.push(cur.clone());
            return;
        }
        for i in 0..nums.len() {
            if cur.contains(&nums[i]) {
                continue;
            }
            cur.push(nums[i]);
            backtrace(nums, cur, res);
            cur.pop();
        }
    }
    let mut res = vec![];
    backtrace(&nums, &mut vec![], &mut res);
    return res;
}

#[test]
pub fn t1() {
    assert_eq!(permute(vec![0, 1]), vec![vec![0, 1], vec![1, 0]]);
}
