// https://leetcode.com/problems/path-sum-ii
// 
// Given a binary tree and a sum, find all root-to-leaf paths where each path's sum equals the given sum.
// 
// For example:
// Given the below binary tree and `sum = 22`,
// ```
//               5
//              / \
//             4   8
//            /   / \
//           11  13  4
//          /  \    / \
//         7    2  5   1
// ```
// 
// return
// 
// ```
// [
//    [5,4,11,2],
//    [5,8,4,5]
// ]
// ```

/**
 * Definition for a binary tree node.
 * function TreeNode(val) {
 *     this.val = val;
 *     this.left = this.right = null;
 * }
 */
/**
 * @param {TreeNode} root
 * @param {number} sum
 * @return {number[][]}
 */
var pathSum = function(root, sum) {
  const helper = (node, temp, sum) => {
    if (node === null) {
      return
    }

    temp.push(node.val)
    sum = sum - node.val

    if (node.left === null && node.right === null) {
      if (sum === 0) {
        r.push([...temp])
      }
    }

    helper(node.right, temp, sum)
    helper(node.left, temp, sum)

    temp.pop()
  }

  let r = []
  let temp = []
  helper(root, temp, sum)
  return r
};