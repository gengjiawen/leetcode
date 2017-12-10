# https://leetcode.com/problems/remove-duplicates-from-sorted-array-ii
# 
# Follow up for "Remove Duplicates":
# What if duplicates are allowed at most _twice_?
# For example,
# Given sorted array _nums_ = `[1,1,1,2,2,3]`,
# Your function should return length = `5`, with the first five elements of _nums_ being `1`, `1`, `2`, `2` and `3`. It doesn't matter what you leave beyond the new length.

class Solution(object):
    def removeDuplicates(self, nums):
        """
        :type nums: List[int]
        :rtype: int
        """
        if len(nums) < 3:
            return len(nums)

        j = 1
        for i in range(2, len(nums)):
            print(i)
            if nums[i] != nums[j-1]:
                j = j + 1
                nums[j] = nums[i]
        return j + 1


    def removeDuplicates2(self, nums):
        """
        :type nums: List[int]
        :rtype: int
        """
        a = list()
        j = 0
        for i in range(len(nums)):
            if nums[i] not in a:
                if len(a) != 0:
                    a.clear()
                a.append(nums[i])
                nums[j] = nums[i]
                j = j + 1
            elif len(a) < 2:
                a.append(nums[i])
                nums[j] = nums[i]
                j = j + 1
            else:
                continue
        return j


print(Solution().removeDuplicates([1, 1]))
print(Solution().removeDuplicates([1, 2, 2]))
print(Solution().removeDuplicates([1, 1, 1, 2]))