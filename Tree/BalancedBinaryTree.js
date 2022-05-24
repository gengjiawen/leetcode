// https://leetcode.com/problems/balanced-binary-tree
//
// Given a binary tree, determine if it is height-balanced.
//
// For this problem, a height-balanced binary tree is defined as:
//
// > a binary tree in which the depth of the two subtrees of _every_ node never differ by more than 1.
//
// **Example 1:**
//
// Given the following tree `[3,9,20,null,null,15,7]`:
//
// ```
//     3
//    / \
//   9  20
//     /  \
//    15   7
// ```
//
// Return true.
//
// **Example 2:**
//
// Given the following tree `[1,2,2,3,3,null,null,4,4]`:
//
// ```
//        1
//       / \
//      2   2
//     / \
//    3   3
//   / \
//  4   4
// ```
//
// Return false.

/**
 * Definition for a binary tree node.
 * function TreeNode(val) {
 *     this.val = val;
 *     this.left = this.right = null;
 * }
 */
/**
 * @param {TreeNode} root
 * @return {boolean}
 */
var isBalanced = function (root) {
  const getHeight = (node) => {
    if (node === null) {
      return 0
    }

    return 1 + Math.max(getHeight(node.left), getHeight(node.right))
  }

  if (root === null) {
    return true
  }

  if (Math.abs(getHeight(root.left) - getHeight(root.right)) > 1) {
    return false
  }

  return isBalanced(root.left) && isBalanced(root.right)
}
