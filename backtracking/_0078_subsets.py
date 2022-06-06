# https://leetcode.com/problems/subsets
#
# Given an integer array `nums` of **unique** elements, return _all possible subsets (the power set)_.
#
# The solution set **must not** contain duplicate subsets. Return the solution in **any order**.
#
# **Example 1:**
#
# ```
# **Input:** nums = [1,2,3]
# **Output:** [[],[1],[2],[1,2],[3],[1,3],[2,3],[1,2,3]]
# ```
#
# **Example 2:**
#
# ```
# **Input:** nums = [0]
# **Output:** [[],[0]]
# ```
#
# **Constraints:**
#
# *   `1 <= nums.length <= 10`
# *   `-10 <= nums[i] <= 10`
# *   All the numbers of `nums` are **unique**.


from typing import List


def subsets(nums: List[int]) -> List[List[int]]:
    res = []
    subset = []

    def dfs(i):
        if i >= len(nums):
            print(subset)
            res.append(subset.copy())
            return

        subset.append(nums[i])
        dfs(i + 1)

        subset.pop()
        dfs(i + 1)

    dfs(0)
    return res

if __name__ == '__main__':
    print(subsets([1, 2, 3]))
