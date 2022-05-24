// https://leetcode.com/problems/symmetric-tree
//
// Given a binary tree, check whether it is a mirror of itself (ie, symmetric around its center).
// For example, this binary tree `[1,2,2,3,4,4,3]` is symmetric:
// ```
//  1
// / \
// 2   2
// / \ / \
// 3  4 4  3
// ```
// But the following `[1,2,2,null,3,null,3]` is not:
// ```
//  1
// / \
// 2   2
// \   \
// 3    3
// ```
// **Note:**
// Bonus points if you could solve it both recursively and iteratively.

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
var isSymmetric = function (root) {
  const isSym = (a, b) => {
    if (a === null && b === null) {
      return true
    }

    if ((a === null && b !== null) || (b === null && a !== null)) {
      return false
    }

    if (a.val !== b.val) {
      return false
    }

    return isSym(a.left, b.right) && isSym(a.right, b.left)
  }

  if (root === null) {
    return true
  }

  return isSym(root.left, root.right)
}
