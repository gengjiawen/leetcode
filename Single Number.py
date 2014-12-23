import collections
import heapq
# Given an array of integers, every element appears twice except for one. Find that single one.
#
# Note:
# Your algorithm should have a linear runtime complexity. Could you implement it without using extra memory?


class Solution:
    # @param A, a list of integer
    # @return an integer
    def singleNumber(self, A):
        words_counts = collections.Counter(A)
        odd = min(zip(words_counts.values(), words_counts.keys()))
        print(odd)
        return odd[1]


s = Solution()
# odd = heapq.nsmallest(1, words_counts, key=lambda x: x[1])
l = [5, 2, 2, 3, 3]
print(s.singleNumber(l))
# m = [17, 12, 5, -6, 12, 4, 17, -5, 2, -3, 2, 4, 5, 16, -3, -4, 15, 15, -4, -5, -6]
# print(s.singleNumber(m))
