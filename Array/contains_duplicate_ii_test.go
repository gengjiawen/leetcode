// https://leetcode.com/problems/contains-duplicate-ii
//
// Given an array of integers and an integer _k_, find out whether there are two distinct indices _i_ and _j_ in the array such that **nums[i] = nums[j]** and the **absolute** difference between _i_ and _j_ is at most _k_.
package Array

import "testing"

func TestContainDuplicateII(t *testing.T) {
	got := containsNearbyDuplicate([]int{1, 2, 3, 1}, 3)

	if !got {
		t.Errorf("faied")
	}
}
