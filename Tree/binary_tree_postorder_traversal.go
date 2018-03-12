// https://leetcode.com/problems/binary-tree-postorder-traversal
//
// Given a binary tree, return the _postorder_ traversal of its nodes' values.
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
// return `[3,2,1]`.
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
func postorderTraversal(root *TreeNode) []int {
	res := []int{}
	if root != nil {
		if root.Left != nil {
			res = append(res, postorderTraversal(root.Left)...)
		}
		if root.Right != nil {
			res = append(res, postorderTraversal(root.Right)...)
		}
		res = append(res, root.Val)
	}

	return res
}
