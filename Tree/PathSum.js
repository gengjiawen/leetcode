// https://leetcode.com/problems/path-sum
//
// Given a binary tree and a sum, determine if the tree has a root-to-leaf path such that adding up all the values along the path equals the given sum.
//
// For example:
// Given the below binary tree and `sum = 22`,
//
// ```
//               5
//              / \
//             4   8
//            /   / \
//           11  13  4
//          /  \      \
//         7    2      1
// ```
//
// return true, as there exist a root-to-leaf path `5->4->11->2` which sum is 22.

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
 * @return {boolean}
 */
var hasPathSum = function(root, sum) {
  const f = (node, currentsSum, target) => {
    if (node === null) {
      return false
    }

    currentsSum = currentsSum + node.val
    if (node.left === null && node.right === null) {
      return currentsSum === target
    }

    const leftPart = f(node.left, currentsSum, target)
    const rightPart = f(node.right, currentsSum, target)

    return leftPart || rightPart
  }

  if (root === null) {
    return false
  }

  return f(root, 0, sum)
}
