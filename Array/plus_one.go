// https://leetcode.com/problems/plus-one
//
// Given a non-negative integer represented as a **non-empty** array of digits, plus one to the integer.
//
// You may assume the integer do not contain any leading zero, except the number 0 itself.
//
// The digits are stored such that the most significant digit is at the head of the list.

package Array

func plusOne(digits []int) []int {
	for i := len(digits) - 1; i >= 0; i-- {
		if digits[i] == 9 {
			digits[i] = 0
		} else {
			digits[i] = digits[i] + 1
			return digits
		}
	}

	if digits[0] == 0 {
		return append([]int{1}, digits...)
	}

	return digits
}
