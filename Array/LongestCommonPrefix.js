// https://leetcode.com/problems/longest-common-prefix
// 
// Write a function to find the longest common prefix string amongst an array of strings.

/**
 * @param {string[]} strs
 * @return {string}
 */
var longestCommonPrefix = function(strs) {
  if (strs === null || strs.length === 0) {
    return ""
  }
  const min = Math.min(...strs.map(i => i.length))
  console.log(min);
  for (let i = 0; i < min; i++ ) {
    for (let j = 1; j < strs.length; j++ ) {
      if (strs[j][i] !== strs[0][i]) {
        return strs[j].substr(0, j)
      }
    }
  }

  return strs[0].substr(0, min)
};
