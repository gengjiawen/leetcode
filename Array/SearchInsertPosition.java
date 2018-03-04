// https://leetcode.com/problems/search-insert-position/description/

// Given a sorted array and a target value, return the index if the target is found. If not, return the index where it would be if it were inserted in order.

// You may assume no duplicates in the array.

// Example 1:

// Input: [1,3,5,6], 5
// Output: 2

// Example 2:

// Input: [1,3,5,6], 2
// Output: 1

// Example 3:

// Input: [1,3,5,6], 7
// Output: 4

// Example 1:

// Input: [1,3,5,6], 0
// Output: 0

class Solution {
    public int searchInsert(int[] nums, int target) {
        int start = 0;
        int end = nums.length - 1;
        while (start <= end) {
            int middle = start + (end - start) / 2;
            if (target == nums[middle] ) {
                return middle;
            } else if (target < nums[middle] ) {
                end = middle - 1;
            } else {
                start = middle + 1;
            }
        }

        return start;
    }
}
