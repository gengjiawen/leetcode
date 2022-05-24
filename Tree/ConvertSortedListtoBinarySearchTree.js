// https://leetcode.com/problems/convert-sorted-list-to-binary-search-tree
//
// Given a singly linked list where elements are sorted in ascending order, convert it to a height balanced BST.
//
// For this problem, a height-balanced binary tree is defined as a binary tree in which the depth of the two subtrees of _every_ node never differ by more than 1.
//
// **Example:**
//
// ```
// Given the sorted linked list: [-10,-3,0,5,9],
//
// One possible answer is: [0,-3,9,-10,null,5], which represents the following height balanced BST:
//
//       0
//      / \
//    -3   9
//    /   /
//  -10  5
// ```

/**
 * Definition for singly-linked list.
 * function ListNode(val) {
 *     this.val = val;
 *     this.next = null;
 * }
 */
/**
 * Definition for a binary tree node.
 * function TreeNode(val) {
 *     this.val = val;
 *     this.left = this.right = null;
 * }
 */
import { Treenode } from './ModelDefinition/TreeNode'

/**
 * @param {ListNode} head
 * @return {TreeNode}
 */
var sortedListToBST = function (head) {
  const buildBST = (start, end) => {
    if (start === end) {
      return null
    }

    let fast = start
    let slow = start
    while (fast !== end && fast.next !== end) {
      fast = fast.next.next
      slow = slow.next
    }

    const node = new TreeNode(slow.val)
    node.left = buildBST(start, slow)
    node.right = buildBST(slow.next, end)
    return node
  }

  return buildBST(head, null)
}
