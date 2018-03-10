// https://leetcode.com/problems/binary-tree-inorder-traversal
//
// Given a binary tree, return the _inorder_ traversal of its nodes' values.
//
// For example:
// Given binary tree `[1,null,2,3]`,
//
// ```
//    1
//     \
//      2
//     /
//    3
// ```
//
// return `[1,3,2]`.
//
// **Note:** Recursive solution is trivial, could you do it iteratively?

/**
 * Definition for a binary tree node.
 * function TreeNode(val) {
 *     this.val = val;
 *     this.left = this.right = null;
 * }
 */
/**
 * @param {TreeNode} root
 * @return {number[]}
 */
var inorderTraversal = function(root) {
  const r = []
  if (root !== null) {
    if (root.left !== null) {
        r.push(...inorderTraversal(root.left))
    }
    r.push(root.val)
    if (root.right !== null) {
        r.push(...inorderTraversal(root.right))
    }
  }

  return r;
}
