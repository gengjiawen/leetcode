// https://leetcode.com/problems/binary-tree-zigzag-level-order-traversal
//
// Given a binary tree, return the _zigzag level order_ traversal of its nodes' values. (ie, from left to right, then right to left for the next level and alternate between).
//
// For example:
// Given binary tree `[3,9,20,null,null,15,7]`,
//
// ```
//     3
//    / \
//   9  20
//     /  \
//    15   7
// ```
//
// return its zigzag level order traversal as:
//
// ```
// [
//   [3],
//   [20,9],
//   [15,7]
// ]
// ```

/**
 * Definition for a binary tree node.
 * function TreeNode(val) {
 *     this.val = val;
 *     this.left = this.right = null;
 * }
 */
/**
 * @param {TreeNode} root
 * @return {number[][]}
 */
var zigzagLevelOrder = function(root) {
    const dfs = (array, node, level) => {
        if (node === null) {
            return null
        }
        if (array.length < level + 1) {
            array.push([])
        }
        array[level].push(node.val)
        dfs(array, node.left, level + 1)
        dfs(array, node.right, level + 1)
    }

    const r = []
    dfs(r, root, 0)

    const zigzag = r.map((i, index) => {
        return index % 2 === 0 ? i : i.reverse()
    })
    return zigzag
}
