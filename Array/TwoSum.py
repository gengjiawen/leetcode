# https://leetcode.com/problems/two-sum/description/

class Solution:
    def twoSumForNosorted(self, nums, target):
        """
        :type nums: List[int]
        :type target: int
        :rtype: List[int]
        """
        lookup = {}
        for i, num in enumerate(nums):
            if target - num in lookup:
                return [lookup[target - num], i]
            lookup[num] = i
        return []


    def twoSumForSorted(self, nums, target):
        """
        :type nums: List[int]
        :type target: int
        :rtype: List[int]
        """
        i = 0
        j = nums.__len__() - 1
        while (i < j):
           sum = nums[i] + nums[j]
           if sum < target:
               i = i + 1
           elif sum > target:
               j = j - 1         
           else:
               return [i, j]
        
        return []


print(Solution().twoSumForSorted((2, 7, 11, 15), 9))
print(Solution().twoSumForNosorted((3, 2, 4), 6))