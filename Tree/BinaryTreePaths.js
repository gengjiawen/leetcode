// https://leetcode.com/problems/binary-tree-paths
//
// Given a binary tree, return all root-to-leaf paths.
//
// For example, given the following binary tree:
//
// ```
//    1
//  /   \
// 2     3
//  \
//   5
// ```
//
// All root-to-leaf paths are:
//
// ```
// ["1->2->5", "1->3"]```
//
// **Credits:**
// Special thanks to [@jianchao.li.fighter](https://leetcode.com/discuss/user/jianchao.li.fighter) for adding this problem and creating all test cases.

/**
 * Definition for a binary tree node.
 * function TreeNode(val) {
 *     this.val = val;
 *     this.left = this.right = null;
 * }
 */
/**
 * @param {TreeNode} root
 * @return {string[]}
 */
var binaryTreePaths = function(root) {
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
    return []
  }

  helper(root, [])

  return r.map(i => i.join("->"))
}
