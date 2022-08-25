// https://leetcode.com/problems/reverse-linked-list
//
// Given the `head` of a singly linked list, reverse the list, and return _the reversed list_.
//
// **Example 1:**
//
// ![](https://assets.leetcode.com/uploads/2021/02/19/rev1ex1.jpg)
// ```
// **Input:** head = [1,2,3,4,5]
// **Output:** [5,4,3,2,1]
// ```
//
// **Example 2:**
//
// ![](https://assets.leetcode.com/uploads/2021/02/19/rev1ex2.jpg)
// ```
// **Input:** head = [1,2]
// **Output:** [2,1]
// ```
//
// **Example 3:**
//
// ```
// **Input:** head = []
// **Output:** []
// ```
//
// **Constraints:**
//
// *   The number of nodes in the list is the range `[0, 5000]`.
// *   `-5000 <= Node.val <= 5000`
//
// **Follow up:** A linked list can be reversed either iteratively or recursively. Could you implement both?

use crate::list_node::ListNode;

pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let (mut head, mut prev) = (head, None);
    while let Some(mut node) = head {
        head = node.next.take();
        node.next = prev;
        prev = Some(node);
    }
    return prev;
}

#[test]
pub fn t1() {
    assert_eq!(
        reverse_list(Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 2,
                next: Some(Box::new(ListNode {
                    val: 3,
                    next: Some(Box::new(ListNode {
                        val: 4,
                        next: Some(Box::new(ListNode { val: 5, next: None })),
                    })),
                })),
            })),
        }))),
        Some(Box::new(ListNode {
            val: 5,
            next: Some(Box::new(ListNode {
                val: 4,
                next: Some(Box::new(ListNode {
                    val: 3,
                    next: Some(Box::new(ListNode {
                        val: 2,
                        next: Some(Box::new(ListNode { val: 1, next: None })),
                    })),
                })),
            })),
        }))
    );
}
