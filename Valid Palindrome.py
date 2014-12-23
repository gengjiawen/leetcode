import math
# Given a string, determine if it is a palindrome, considering only alphanumeric characters and ignoring cases.
#
# For example,
# "A man, a plan, a canal: Panama" is a palindrome.
# "race a car" is not a palindrome.
#
# Note:
# Have you consider that the string might be empty? This is a good question to ask during an interview.
#
# For the purpose of this problem, we define empty string as valid palindrome.

class Solution:
    # @param s, a string
    # @return a boolean
    def isPalindrome(self, s):
        if s == "":
            return True
        s = s.lower()
        a = [m for m in s if m.isalnum()]
        l = int(math.ceil(len(a) / 2.0))
        for i in range(l):
            if a[i] != a[len(a) - i - 1]:
                return False

        return True

# s = Solution()
# print(s.isPalindrome(""))
# print(s.isPalindrome("1a2"))
# print(s.isPalindrome("A man, a plan, a canal: Panama"))
# print(s.isPalindrome("race a car"))

