// https://leetcode.com/problems/invert-binary-tree
// 
// Invert a binary tree.
// ```
//      4
//    /   \
//   2     7
//  / \   / \
// 1   3 6   9
//
// to
//
//      4
//    /   \
//   7     2
//  / \   / \
// 9   6 3   1
// **Trivia:**
// This problem was inspired by [this original tweet](https://twitter.com/mxcl/status/608682016205344768) by [Max Howell](https://twitter.com/mxcl):
// > Google: 90% of our engineers use the software you wrote (Homebrew), but you can’t invert a binary tree on a whiteboard so fuck off.

/**
 * Definition for a binary tree node.
 * function TreeNode(val) {
 *     this.val = val;
 *     this.left = this.right = null;
 * }
 */
/**
 * @param {TreeNode} root
 * @return {TreeNode}
 */
var invertTree = function(root) {
  if (root === null || (root.right === null && root.left === null)) {
    return root;
  }

  let temp = root.right
  root.right = invertTree(root.left)
  root.left = invertTree(temp)

  return root
};