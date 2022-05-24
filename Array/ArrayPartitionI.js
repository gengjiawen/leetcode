// https://leetcode.com/problems/array-partition-i
//
// Given an array of **2n** integers, your task is to group these integers into **n** pairs of integer, say (a<sub>1</sub>, b<sub>1</sub>), (a<sub>2</sub>, b<sub>2</sub>), ..., (a<sub>n</sub>, b<sub>n</sub>) which makes sum of min(a<sub>i</sub>, b<sub>i</sub>) for all i from 1 to n as large as possible.
//
// **Example 1:**
//
// ```
// **Input:** [1,4,3,2]
//
// **Output:** 4
// **Explanation:** n is 2, and the maximum sum of pairs is 4 = min(1, 2) + min(3, 4).
// ```
//
// **Note:**
//
// 1.  **n** is a positive integer, which is in the range of [1, 10000].
// 2.  All the integers in the array will be in the range of [-10000, 10000].

/**
 * @param {number[]} nums
 * @return {number}
 */
var arrayPairSum = function (nums) {
  nums = nums.sort((a, b) => a - b)
  let sum = 0
  for (i = 0; i < nums.length; i = i + 2) {
    sum = sum + nums[i]
  }
  return sum
}
