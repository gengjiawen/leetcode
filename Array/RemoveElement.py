# https://leetcode.com/problems/remove-element
# 
# Given an array and a value, remove all instances of that value in place and return the new length.
# Do not allocate extra space for another array, you must do this in place with constant memory.
# The order of elements can be changed. It doesn't matter what you leave beyond the new length.
# **Example:**
# Given input array _nums_ = `[3,2,2,3]`, _val_ = `3`
# Your function should return length = 2, with the first two elements of _nums_ being 2.

class Solution(object):
    def removeElement(self, nums, val):
        """
        :type nums: List[int]
        :type val: int
        :rtype: int
        """
        i = 0
        j = 0
        for i in range(len(nums)):
            if nums[i] == val:
                continue
            nums[j] = nums[i]
            j = j + 1

        return j
        