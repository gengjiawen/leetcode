// https://leetcode.com/problems/sort-even-and-odd-indices-independently
//
// You are given a **0-indexed** integer array `nums`. Rearrange the values of `nums` according to the following rules:
//
// 1.  Sort the values at **odd indices** of `nums` in **non-increasing** order.
//     *   For example, if `nums = [4,**<u>1</u>**,2,<u>**3**</u>]` before this step, it becomes `[4,<u>**3**</u>,2,**<u>1</u>**]` after. The values at odd indices `1` and `3` are sorted in non-increasing order.
// 2.  Sort the values at **even indices** of `nums` in **non-decreasing** order.
//     *   For example, if `nums = [<u>**4**</u>,1,<u>**2**</u>,3]` before this step, it becomes `[<u>**2**</u>,1,<u>**4**</u>,3]` after. The values at even indices `0` and `2` are sorted in non-decreasing order.
//
// Return _the array formed after rearranging the values of_ `nums`.
//
// **Example 1:**
//
// ```
// **Input:** nums = [4,1,2,3]
// **Output:** [2,3,4,1]
// **Explanation:**
// First, we sort the values present at odd indices (1 and 3) in non-increasing order.
// So, nums changes from [4,**<u>1</u>**,2,**<u>3</u>**] to [4,<u>**3**</u>,2,**<u>1</u>**].
// Next, we sort the values present at even indices (0 and 2) in non-decreasing order.
// So, nums changes from [<u>**4**</u>,1,**<u>2</u>**,3] to [<u>**2**</u>,3,<u>**4**</u>,1].
// Thus, the array formed after rearranging the values is [2,3,4,1].
// ```
//
// **Example 2:**
//
// ```
// **Input:** nums = [2,1]
// **Output:** [2,1]
// **Explanation:**
// Since there is exactly one odd index and one even index, no rearrangement of values takes place.
// The resultant array formed is [2,1], which is the same as the initial array.
// ```
//
// **Constraints:**
//
// *   `1 <= nums.length <= 100`
// *   `1 <= nums[i] <= 100`

pub fn sort_even_odd(nums: Vec<i32>) -> Vec<i32> {
    let mut evens = nums.iter().step_by(2).collect::<Vec<&i32>>();
    let mut odds = nums[1..].iter().step_by(2).collect::<Vec<&i32>>();
    evens.sort_unstable();
    odds.sort_unstable_by_key(|&x| -x);
    let mut res = evens
        .iter()
        .zip(odds.iter())
        .map(|(&x, &y)| vec![*x, *y])
        .flatten()
        .collect::<Vec<i32>>();
    if evens.len() > odds.len() {
        res.push(*evens[evens.len() - 1]);
    }
    return res;
}

#[test]
pub fn t1() {
    assert_eq!(sort_even_odd(vec![4, 1, 2, 3]), vec![2, 3, 4, 1]);
    assert_eq!(sort_even_odd(vec![2, 1, 2]), vec![2, 1, 2]);
}
