// https://leetcode.com/problems/binary-tree-preorder-traversal
// 
// Given a binary tree, return the _preorder_ traversal of its nodes' values.
// 
// For example:
// Given binary tree `[1,null,2,3]`,
// 
// ```
//    1
//     \
//      2
//     /
//    3
// ```
// 
// return `[1,2,3]`.
// 
// **Note:** Recursive solution is trivial, could you do it iteratively?

/**
 * Definition for a binary tree node.
 * type TreeNode struct {
 *     Val int
 *     Left *TreeNode
 *     Right *TreeNode
 * }
 */
package Tree
func preorderTraversal(root *TreeNode) []int {
    res := []int{}
    if (root != nil){
    	res = append(res, root.Val)
    	if root.Left != nil {
			res = append(res, preorderTraversal(root.Left)...)
		}
		if root.Right != nil {
			res = append(res, preorderTraversal(root.Right)...)
		}
    }

    return res
}