// https://leetcode.com/problems/construct-binary-tree-from-inorder-and-postorder-traversal
//
// Given inorder and postorder traversal of a tree, construct the binary tree.
//
// **Note:**
// You may assume that duplicates do not exist in the tree.
//
// For example, given
//
// ```
// inorder = [9,3,15,20,7]
// postorder = [9,15,7,20,3]```
//
// Return the following binary tree:
//
// ```
//     3
//    / \
//   9  20
//     /  \
//    15   7
// ```

/**
 * Definition for a binary tree node.
 * function TreeNode(val) {
 *     this.val = val;
 *     this.left = this.right = null;
 * }
 */
/**
 * @param {number[]} inorder
 * @param {number[]} postorder
 * @return {TreeNode}
 */
var buildTree = function(inorder, postorder) {
  if (postorder.length === 0) {
    return null
  }

  const root = postorder[postorder.length - 1]
  const node = new TreeNode(root)

  const index = inorder.findIndex(i => i === root)
  const leftInOrder = inorder.slice(0, index)
  const rightInOrder = inorder.slice(index + 1, inorder.length)

  const leftInPostOrder = postorder.slice(0, leftInOrder.length)
  const rightPostOrder = postorder.slice(
    leftInOrder.length + 1,
    postorder.length - 1
  )

  node.left = buildTree(leftInOrder, leftInPostOrder)
  node.right = buildTree(rightInOrder, rightPostOrder)

  return node
}
