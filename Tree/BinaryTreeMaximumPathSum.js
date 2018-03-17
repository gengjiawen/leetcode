// https://leetcode.com/problems/binary-tree-maximum-path-sum
//
// Given a binary tree, find the maximum path sum.
//
// For this problem, a path is defined as any sequence of nodes from some starting node to any node in the tree along the parent-child connections. The path must contain **at least one node** and does not need to go through the root.
//
// For example:
// Given the below binary tree,
//
// ```
//        1
//       / \
//      2   3
// ```
//
// Return `6`.

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
var maxPathSum = function(root) {
  const findMax = node => {
    if (node === null) {
      return 0
    }
    let sum = node.val
    const l = findMax(node.left)
    const r = findMax(node.right)
    if (l > 0) {
      sum = sum + l
    }
    if (r > 0) {
      sum = sum + r
    }

    maxSum = Math.max(sum, maxSum)

    console.log(node.val, l, r, sum)
    return Math.max(l, r) > 0 ? Math.max(r, l) + node.val : node.val
  }

  let maxSum = Number.MIN_SAFE_INTEGER
  findMax(root)
  return maxSum
}
