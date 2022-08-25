// https://leetcode.com/problems/merge-two-sorted-lists
//
// You are given the heads of two sorted linked lists `list1` and `list2`.
//
// Merge the two lists in a one **sorted** list. The list should be made by splicing together the nodes of the first two lists.
//
// Return _the head of the merged linked list_.
//
// **Example 1:**
//
// ![](https://assets.leetcode.com/uploads/2020/10/03/merge_ex1.jpg)
// ```
// **Input:** list1 = [1,2,4], list2 = [1,3,4]
// **Output:** [1,1,2,3,4,4]
// ```
//
// **Example 2:**
//
// ```
// **Input:** list1 = [], list2 = []
// **Output:** []
// ```
//
// **Example 3:**
//
// ```
// **Input:** list1 = [], list2 = [0]
// **Output:** [0]
// ```
//
// **Constraints:**
//
// *   The number of nodes in both lists is in the range `[0, 50]`.
// *   `-100 <= Node.val <= 100`
// *   Both `list1` and `list2` are sorted in **non-decreasing** order.
use crate::list_node::ListNode;

pub fn merge_two_lists(
    list1: Option<Box<ListNode>>,
    list2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    let mut list1 = list1.as_ref();
    let mut list2 = list2.as_ref();
    let mut start = Box::new(ListNode::new(0));
    let mut end = &mut start;
    while let (Some(n1), Some(n2)) = (list1, list2) {
        if n1.val < n2.val {
            end.next = list1.cloned();
            list1 = n1.next.as_ref();
        } else {
            end.next = list2.cloned();
            list2 = n2.next.as_ref();
        }
        end = end.next.as_mut().unwrap();
    }
    if list1.is_some() {
        end.next = list1.cloned();
    } else if list2.is_some() {
        end.next = list2.cloned();
    }
    return start.next;
}

#[test]
pub fn t1() {
    assert_eq!(
        merge_two_lists(
            Some(Box::new(ListNode {
                val: 1,
                next: Some(Box::new(ListNode {
                    val: 2,
                    next: Some(Box::new(ListNode { val: 4, next: None })),
                })),
            })),
            Some(Box::new(ListNode {
                val: 1,
                next: Some(Box::new(ListNode {
                    val: 3,
                    next: Some(Box::new(ListNode { val: 4, next: None })),
                })),
            })),
        ),
        Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 1,
                next: Some(Box::new(ListNode {
                    val: 2,
                    next: Some(Box::new(ListNode {
                        val: 3,
                        next: Some(Box::new(ListNode {
                            val: 4,
                            next: Some(Box::new(ListNode { val: 4, next: None })),
                        })),
                    })),
                })),
            })),
        }))
    );
}
