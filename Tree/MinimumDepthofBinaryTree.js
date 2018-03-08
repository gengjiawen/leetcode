// https://leetcode.com/problems/minimum-depth-of-binary-tree
//
// Given a binary tree, find its minimum depth.
//
// The minimum depth is the number of nodes along the shortest path from the root node down to the nearest leaf node.

/**
 * Definition for a binary tree node.
 * function TreeNode(val) {
 *     this.val = val;
 *     this.left = this.right = null;
 * }
 */
/**
 * @param {TreeNode} root
 * @return {number}
 */
var minDepth = function(root) {
  if (root === null) {
    return 0
  }
  if (root.left === null) {
    return 1 + minDepth(root.right)
  }
  if (root.right === null) {
    return 1 + minDepth(root.left)
  }

  return 1 + Math.min(minDepth(root.left), minDepth(root.right))
}
