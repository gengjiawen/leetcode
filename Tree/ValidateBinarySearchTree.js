// https://leetcode.com/problems/validate-binary-search-tree
// 
// Given a binary tree, determine if it is a valid binary search tree (BST).
// 
// Assume a BST is defined as follows:
// 
// *   The left subtree of a node contains only nodes with keys **less than** the node's key.
// *   The right subtree of a node contains only nodes with keys **greater than** the node's key.
// *   Both the left and right subtrees must also be binary search trees.
// 
// **Example 1:**
// 
// ```
//     2
//    / \
//   1   3
// ```
// Binary tree `[2,1,3]`, return true.
// 
// **Example 2:**
// 
// ```
//     1
//    / \
//   2   3
// ```
// Binary tree `[1,2,3]`, return false.

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
var isValidBST = function(root) {
  const helper = (node, min, max) => {
    if (node === null) {
      return true
    }
    if (node.val <= min || node.val >= max) {
      return false
    }

    return helper(node.left, min, node.val) && helper(node.right, node.val, max)
  }

  return helper(root, Number.MIN_SAFE_INTEGER, Number.MAX_SAFE_INTEGER)

};