// https://leetcode.com/problems/sum-root-to-leaf-numbers
//
// Given a binary tree containing digits from `0-9` only, each root-to-leaf path could represent a number.
//
// An example is the root-to-leaf path `1->2->3` which represents the number `123`.
//
// Find the total sum of all root-to-leaf numbers.
//
// For example,
//
// ```
//     1
//    / \
//   2   3
// ```
//
// The root-to-leaf path `1->2` represents the number `12`.
// The root-to-leaf path `1->3` represents the number `13`.
//
// Return the sum = 12 + 13 = `25`.

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
var sumNumbers = function(root) {
  const helper = (node, result) => {
    if (node === null) {
      return null
    }

    result.push(node.val)
    if (node.left === null && node.right === null) {
      r.push([...result])
    }

    helper(node.left, result)
    helper(node.right, result)

    result.pop()
  }

  let r = []
  if (root === null) {
    return 0
  }

  helper(root, [])

  return r.map(i => i.join('')).reduce((a, b) => parseInt(b) + a, 0)
}
