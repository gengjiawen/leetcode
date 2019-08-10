// https://leetcode.com/problems/contains-duplicate
//
// Given an array of integers, find if the array contains any duplicates. Your function should return true if any value appears at least twice in the array, and it should return false if every element is distinct.
package Array

func containsDuplicate(nums []int) bool {
	m := map[int]bool{}

	for i := range nums {
		if m[nums[i]] {
			return true
		}

		m[nums[i]] = true
	}

	return false
}
