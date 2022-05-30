// https://leetcode.com/problems/top-k-frequent-elements
//
// Given an integer array `nums` and an integer `k`, return _the_ `k` _most frequent elements_. You may return the answer in **any order**.
//
// **Example 1:**
//
// ```
// **Input:** nums = [1,1,1,2,2,3], k = 2
// **Output:** [1,2]
// ```
//
// **Example 2:**
//
// ```
// **Input:** nums = [1], k = 1
// **Output:** [1]
// ```
//
// **Constraints:**
//
// *   `1 <= nums.length <= 10<sup>5</sup>`
// *   `k` is in the range `[1, the number of unique elements in the array]`.
// *   It is **guaranteed** that the answer is **unique**.
//
// **Follow up:** Your algorithm's time complexity must be better than `O(n log n)`, where n is the array's size.

/// bucket sort solution https://www.youtube.com/watch?v=YPTqKIgVk-k
pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
    let mut count = std::collections::HashMap::new();
    for i in 0..nums.len() {
        *count.entry(&nums[i]).or_insert(0) += 1;
    }

    let mut bucket = vec![Vec::new(); nums.len() + 1];
    for (&key, &value) in count.iter() {
        bucket[value as usize].push(*key);
    }
    return bucket
        .into_iter()
        .rev()
        .flatten()
        .take(k as usize)
        .collect();
}

#[test]
fn test() {
    assert_eq!(top_k_frequent(vec![1, 1, 1, 2, 2, 3], 2), vec![1, 2]);
}
