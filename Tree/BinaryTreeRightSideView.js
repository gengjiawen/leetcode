// https://leetcode.com/problems/binary-tree-right-side-view
//
// Given a binary tree, imagine yourself standing on the _right_ side of it, return the values of the nodes you can see ordered from top to bottom.
//
// For example:
// Given the following binary tree,
//
// ```
//    1            <---
//  /   \
// 2     3         <---
//  \     \
//   5     4       <---
// ```
//
// You should return `[1, 3, 4]`.
//
// **Credits:**
// Special thanks to [@amrsaqr](https://leetcode.com/discuss/user/amrsaqr) for adding this problem and creating all test cases.

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
var rightSideView = function (root) {
  const dfs = (array, node, level) => {
    if (node === null) {
      return null
    }
    if (array.length < level + 1) {
      array.push([])
    }
    array[level] = [node.val]
    dfs(array, node.left, level + 1)
    dfs(array, node.right, level + 1)
  }

  const r = []
  dfs(r, root, 0)
  return r.reduce((a, b) => a.concat(b), [])
}
