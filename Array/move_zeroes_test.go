// https://leetcode.com/problems/move-zeroes
//
// Given an array `nums`, write a function to move all `0`'s to the end of it while maintaining the relative order of the non-zero elements.
//
// For example, given `nums = [0, 1, 0, 3, 12]`, after calling your function, `nums` should be `[1, 3, 12, 0, 0]`.
//
// **Note**:
//
// 1.  You must do this **in-place** without making a copy of the array.
// 2.  Minimize the total number of operations.
//
// **Credits:**
// Special thanks to [@jianchao.li.fighter](https://leetcode.com/discuss/user/jianchao.li.fighter) for adding this problem and creating all test cases.

package Array

import (
	"reflect"
	"testing"
)

func TestMoveZeros(t *testing.T) {
	numbers := []int{0, 1, 0, 3, 12}
	moveZeroes(numbers)
	want := []int{1, 3, 12, 0, 0}
	if !reflect.DeepEqual(numbers, want) {
		t.Errorf("got %d, want %d", numbers, want)
	}
}
