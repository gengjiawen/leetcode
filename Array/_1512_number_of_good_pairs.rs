// https://leetcode.com/problems/number-of-good-pairs
//
// Given an array of integers `nums`, return _the number of **good pairs**_.
//
// A pair `(i, j)` is called _good_ if `nums[i] == nums[j]` and `i` < `j`.
//
// **Example 1:**
//
// ```
// **Input:** nums = [1,2,3,1,1,3]
// **Output:** 4
// **Explanation:** There are 4 good pairs (0,3), (0,4), (3,4), (2,5) 0-indexed.
// ```
//
// **Example 2:**
//
// ```
// **Input:** nums = [1,1,1,1]
// **Output:** 6
// **Explanation:** Each pair in the array are _good_.
// ```
//
// **Example 3:**
//
// ```
// **Input:** nums = [1,2,3]
// **Output:** 0
// ```
//
// **Constraints:**
//
// *   `1 <= nums.length <= 100`
// *   `1 <= nums[i] <= 100`

pub fn num_identical_pairs(nums: Vec<i32>) -> i32 {
    let mut res = 0;
    for i in 0..nums.len() {
        for j in i + 1..nums.len() {
            if nums[i] == nums[j] {
                res += 1;
            }
        }
    }
    return res;
}

pub fn num_identical_pairs_fast(nums: Vec<i32>) -> i32 {
    let mut map = std::collections::HashMap::new();
    for i in 0..nums.len() {
        *map.entry(nums[i]).or_insert(0) += 1;
    }
    let mut res = 0;
    for &i in map.values() {
        if i >= 2 {
            res += i * (i - 1) / 2;
        }
    }

    return res;
}

#[test]
pub fn t1() {
    // assert_eq!(num_identical_pairs(vec![1, 2, 3, 1, 1, 3]), 4);
    assert_eq!(num_identical_pairs_fast(vec![1, 2, 3, 1, 1, 3]), 4);
}
