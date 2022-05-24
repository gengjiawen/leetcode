// https://leetcode.com/problems/longest-consecutive-sequence
//
// Given an unsorted array of integers, find the length of the longest consecutive elements sequence.
//
// For example,
// Given `[100, 4, 200, 1, 3, 2]`,
// The longest consecutive elements sequence is `[1, 2, 3, 4]`. Return its length: `4`.
//
// Your algorithm should run in O(_n_) complexity.

/**
 * @param {number[]} nums
 * @return {number}
 */
var longestConsecutive = function (nums) {
  let maxLength = 0
  const s = new Set(nums)
  for (let i = 0; i < nums.length; i++) {
    if (s.has(nums[i])) {
      s.delete(nums[i])
      let pre = nums[i] - 1
      let next = nums[i] + 1
      while (s.has(pre)) {
        s.delete(pre)
        pre = pre - 1
      }
      while (s.has(next)) {
        s.delete(next)
        next = next + 1
      }
      maxLength = Math.max(maxLength, next - pre - 1)
    }
  }

  return maxLength
}
