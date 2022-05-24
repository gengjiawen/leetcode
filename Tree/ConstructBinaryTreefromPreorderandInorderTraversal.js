// https://leetcode.com/problems/construct-binary-tree-from-preorder-and-inorder-traversal
//
// Given preorder and inorder traversal of a tree, construct the binary tree.
//
// **Note:**
// You may assume that duplicates do not exist in the tree.
//
// For example, given
//
// ```
// preorder = [3,9,20,15,7]
// inorder = [9,3,15,20,7]```
//
// Return the following binary tree:
//
// ```
//     3
//    / \
//   9  20
//     /  \
//    15   7```

/**
 * Definition for a binary tree node.
 * function TreeNode(val) {
 *     this.val = val;
 *     this.left = this.right = null;
 * }
 */
/**
 * @param {number[]} preorder
 * @param {number[]} inorder
 * @return {TreeNode}
 */
var buildTree = function (preorder, inorder) {
  if (preorder.length === 0) {
    return null
  }

  const root = preorder[0]
  const node = new TreeNode(root)

  const index = inorder.findIndex((i) => i === root)
  const leftInOrder = inorder.slice(0, index)
  const rightInOrder = inorder.slice(index + 1, inorder.length - 1)

  const leftPreOrder = preorder.slice(1, leftInOrder.length + 1)
  const rightPreOrder = preorder.slice(leftPreOrder.length + 1, preorder.length)

  node.left = buildTree(leftPreOrder, leftInOrder)
  node.right = buildTree(rightPreOrder, rightInOrder)

  return node
}
