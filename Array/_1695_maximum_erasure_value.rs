// https://leetcode.com/problems/maximum-erasure-value
//
// You are given an array of positive integers `nums` and want to erase a subarray containing **unique elements**.
// The **score** you get by erasing the subarray is equal to the **sum** of its elements.
//
// Return _the **maximum score** you can get by erasing **exactly one** subarray._
//
// An array `b` is called to be a <span class="tex-font-style-it">subarray</span> of `a` if it forms a contiguous subsequence of `a`, that is, if it is equal to `a[l],a[l+1],...,a[r]` for some `(l,r)`.
//
// **Example 1:**
//
// ```
// **Input:** nums = [4,2,4,5,6]
// **Output:** 17
// **Explanation:** The optimal subarray here is [2,4,5,6].
// ```
//
// **Example 2:**
//
// ```
// **Input:** nums = [5,2,1,2,5,2,1,2,5]
// **Output:** 8
// **Explanation:** The optimal subarray here is [5,2,1] or [1,2,5].
// ```
//
// **Constraints:**
//
// *   `1 <= nums.length <= 10<sup>5</sup>`
// *   `1 <= nums[i] <= 10<sup>4</sup>`

pub fn maximum_unique_subarray(nums: Vec<i32>) -> i32 {
    let mut max_sum = 0;
    let mut sum = 0;
    let mut start = 0;
    let mut map = std::collections::HashMap::new();

    for end in 0..nums.len() {
        let num = nums[end];
        sum += num;
        *map.entry(num).or_insert(0) += 1;
        if end - start + 1 == map.len() {
            max_sum = max_sum.max(sum);
        }
        while end - start + 1 > map.len() {
            let head_val = nums[start];
            *map.entry(head_val).or_insert(0) -= 1;
            if map[&head_val] == 0 {
                map.remove(&head_val);
            }
            sum -= head_val;
            start += 1;
        }
    }
    return max_sum;
}

#[test]
pub fn test() {
    assert_eq!(maximum_unique_subarray(vec![4, 2, 4, 5, 6]), 17);
    assert_eq!(maximum_unique_subarray(vec![187, 470, 25, 436, 538, 809, 441, 167, 477, 110, 275, 133, 666, 345, 411, 459, 490, 266, 987, 965, 429, 166, 809, 340, 467, 318, 125, 165, 809, 610, 31, 585, 970, 306, 42, 189, 169, 743, 78, 810, 70, 382, 367, 490, 787, 670, 476, 278, 775, 673, 299, 19, 893, 817, 971, 458, 409, 886, 434]), 16911);
}
