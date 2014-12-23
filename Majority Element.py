# encoding=utf8
from collections import Counter
# Given an array of size n, find the majority element. The majority element is the element that appears more than ⌊ n/2 ⌋ times.
#
# You may assume that the array is non-empty and the majority element always exist in the array.
#
# Credits:
# Special thanks to @ts for adding this problem and creating all test cases.


class Solution:
    # @param num, a list of integers
    # @return an integer
    def majorityElement(self, num):
        words_counts = Counter(num)
        majority = words_counts.most_common(1)
        return majority[0][0]

