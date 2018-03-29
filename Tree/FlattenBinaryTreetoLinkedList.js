// https://leetcode.com/problems/flatten-binary-tree-to-linked-list
//
// Given a binary tree, flatten it to a linked list in-place.
//
// For example,
// Given
//
// ```
//          1
//         / \
//        2   5
//       / \   \
//      3   4   6
// ```
// The flattened tree should look like:
//
// ```
//    1
//     \
//      2
//       \
//        3
//         \
//          4
//           \
//            5
//             \
//              6
// ```
//
// [click to show hints.](#)
//
// **Hints:**
//
// If you notice carefully in the flattened tree, each node's right child points to the next node of a pre-order traversal.

/**
 * Definition for a binary tree node.
 * function TreeNode(val) {
 *     this.val = val;
 *     this.left = this.right = null;
 * }
 */
/**
 * @param {TreeNode} root
 * @return {void} Do not return anything, modify root in-place instead.
 */
var flatten = function(root) {
  const preOrder = node => {
    if (node === null) {
      return
    }
    r.push(node)
    preOrder(node.left)
    preOrder(node.right)
  }

  if (root === null) {
    return
  }

  const r = []
  preOrder(root)
  r.map((i, index) => {
    if (index === r.length - 1) {
      i.left = null
      i.right = null
    } else {
      i.left = null
      i.right = r[index + 1]
    }
  })
}
