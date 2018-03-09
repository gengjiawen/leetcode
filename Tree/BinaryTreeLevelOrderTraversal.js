// https://leetcode.com/problems/binary-tree-level-order-traversal
//
// Given a binary tree, return the _level order_ traversal of its nodes' values. (ie, from left to right, level by level).
//
// For example:
// Given binary tree `[3,9,20,null,null,15,7]`,
//
// ```
//     3
//    / \
//   9  20
//     /  \
//    15   7
// ```
//
// return its level order traversal as:
//
// ```
// [
//   [3],
//   [9,20],
//   [15,7]
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
 * @return {number[][]}
 */
var levelOrder = function(root) {
  const dfs = (array, node, level) => {
    if (node === null) {
      return null
    }
    if (array.length < level + 1) {
      array.push([])
    }
    array[level].push(node.val)
    dfs(array, node.left, level + 1)
    dfs(array, node.right, level + 1)
  }

  const r = []
  dfs(r, root, 0)
  return r
}
