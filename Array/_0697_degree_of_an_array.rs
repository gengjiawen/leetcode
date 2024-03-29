// https://leetcode.com/problems/degree-of-an-array
//
// Given a non-empty array of non-negative integers `nums`, the **degree** of this array is defined as the maximum frequency of any one of its elements.
//
// Your task is to find the smallest possible length of a (contiguous) subarray of `nums`, that has the same degree as `nums`.
//
// **Example 1:**
//
// ```
// **Input:** nums = [1,2,2,3,1]
// **Output:** 2
// **Explanation:**
// The input array has a degree of 2 because both elements 1 and 2 appear twice.
// Of the subarrays that have the same degree:
// [1, 2, 2, 3, 1], [1, 2, 2, 3], [2, 2, 3, 1], [1, 2, 2], [2, 2, 3], [2, 2]
// The shortest length is 2\. So return 2.
// ```
//
// **Example 2:**
//
// ```
// **Input:** nums = [1,2,2,3,1,4,2]
// **Output:** 6
// **Explanation:**
// The degree is 3 because the element 2 is repeated 3 times.
// So [2,2,3,1,4,2] is the shortest subarray, therefore returning 6.
// ```
//
// **Constraints:**
//
// *   `nums.length` will be between 1 and 50,000.
// *   `nums[i]` will be an integer between 0 and 49,999.

pub fn find_shortest_sub_array(nums: Vec<i32>) -> i32 {
    let mut nums_count = std::collections::HashMap::new();
    let mut first_index = std::collections::HashMap::new();
    let mut res = 0;
    let mut degree = 0;
    for i in 0..nums.len() {
        first_index.entry(nums[i]).or_insert(i as i32);
        *nums_count.entry(nums[i]).or_insert(0) += 1;
        if *nums_count.get(&nums[i]).unwrap() > degree {
            degree = *nums_count.get(&nums[i]).unwrap();
            res = i as i32 - first_index.get(&nums[i]).unwrap() + 1;
        } else if *nums_count.get(&nums[i]).unwrap() == degree {
            res = std::cmp::min(res, i as i32 - first_index.get(&nums[i]).unwrap() + 1);
        }
    }
    return res;
}

#[test]
pub fn t1() {
    assert_eq!(find_shortest_sub_array(vec![1, 2, 2, 3, 1]), 2);
}
