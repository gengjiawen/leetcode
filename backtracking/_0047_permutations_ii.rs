// https://leetcode.com/problems/permutations-ii
//
// Given a collection of numbers, `nums`, that might contain duplicates, return _all possible unique permutations **in any order**._
//
// **Example 1:**
//
// ```
// **Input:** nums = [1,1,2]
// **Output:**
// [[1,1,2],
//  [1,2,1],
//  [2,1,1]]
// ```
//
// **Example 2:**
//
// ```
// **Input:** nums = [1,2,3]
// **Output:** [[1,2,3],[1,3,2],[2,1,3],[2,3,1],[3,1,2],[3,2,1]]
// ```
//
// **Constraints:**
//
// *   `1 <= nums.length <= 8`
// *   `-10 <= nums[i] <= 10`

pub fn permute_unique(nums: Vec<i32>) -> Vec<Vec<i32>> {
    fn backtrack(nums: &Vec<i32>, cur: &mut Vec<i32>, used: &mut Vec<bool>, res: &mut Vec<Vec<i32>>) {
        if cur.len() == nums.len() {
            res.push(cur.clone());
            return;
        }
        for i in 0..nums.len() {
            if used[i] || (i > 0 && nums[i] == nums[i - 1] && !used[i - 1]) {
                continue;
            }
            used[i] = true;
            cur.push(nums[i]);
            backtrack(nums, cur, used, res);
            used[i] = false;
            cur.pop();
        }
    }
    let mut res = vec![];
    let mut sorted_num = nums.clone();
    sorted_num.sort();
    backtrack(&sorted_num, &mut vec![], &mut vec![false; nums.len()], &mut res);
    return res;
}

#[test]
pub fn t1() {
    assert_eq!(permute_unique(vec![1, 1, 2]), vec![vec![1, 1, 2], vec![1, 2, 1], vec![2, 1, 1]]);
}

#[test]
pub fn t2() {
    assert_eq!(permute_unique(vec![1, 1, 1]), vec![vec![1, 1, 1]]);
}
