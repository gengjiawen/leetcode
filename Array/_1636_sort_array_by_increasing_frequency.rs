// https://leetcode.com/problems/sort-array-by-increasing-frequency
//
// Given an array of integers `nums`, sort the array in **increasing** order based on the frequency of the values. If multiple values have the same frequency, sort them in **decreasing** order.
//
// Return the _sorted array_.
//
// **Example 1:**
//
// ```
// **Input:** nums = [1,1,2,2,2,3]
// **Output:** [3,1,1,2,2,2]
// **Explanation:** '3' has a frequency of 1, '1' has a frequency of 2, and '2' has a frequency of 3.
// ```
//
// **Example 2:**
//
// ```
// **Input:** nums = [2,3,1,3,2]
// **Output:** [1,3,3,2,2]
// **Explanation:** '2' and '3' both have a frequency of 2, so they are sorted in decreasing order.
// ```
//
// **Example 3:**
//
// ```
// **Input:** nums = [-1,1,-6,4,5,-6,1,4,1]
// **Output:** [5,-1,4,4,-6,-6,1,1,1]```
//
// **Constraints:**
//
// *   `1 <= nums.length <= 100`
// *   `-100 <= nums[i] <= 100`

pub fn frequency_sort(nums: Vec<i32>) -> Vec<i32> {
    let mut map = std::collections::HashMap::new();
    for num in nums.iter() {
        *map.entry(*num).or_insert(0) += 1;
    }
    let mut res = nums.clone();
    res.sort_by_key(|&num| (map[&num], -num));
    return res;
}

#[test]
pub fn t1() {
    assert_eq!(
        frequency_sort(vec![1, 1, 2, 2, 2, 3]),
        vec![3, 1, 1, 2, 2, 2]
    );
    assert_eq!(frequency_sort(vec![2, 3, 1, 3, 2]), vec![1, 3, 3, 2, 2]);
    assert_eq!(
        frequency_sort(vec![-1, 1, -6, 4, 5, -6, 1, 4, 1]),
        vec![5, -1, 4, 4, -6, -6, 1, 1, 1]
    );
}
