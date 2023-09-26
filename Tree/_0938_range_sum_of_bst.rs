// https://leetcode.com/problems/range-sum-of-bst
//
// Given the `root` node of a binary search tree and two integers `low` and `high`, return _the sum of values of all nodes with a value in the **inclusive** range_ `[low, high]`.
//
// **Example 1:**
//
// ![](https://assets.leetcode.com/uploads/2020/11/05/bst1.jpg)
// ```
// **Input:** root = [10,5,15,3,7,null,18], low = 7, high = 15
// **Output:** 32
// **Explanation:** Nodes 7, 10, and 15 are in the range [7, 15]. 7 + 10 + 15 = 32.
// ```
//
// **Example 2:**
//
// ![](https://assets.leetcode.com/uploads/2020/11/05/bst2.jpg)
// ```
// **Input:** root = [10,5,15,3,7,13,18,1,null,6], low = 6, high = 10
// **Output:** 23
// **Explanation:** Nodes 6, 7, and 10 are in the range [6, 10]. 6 + 7 + 10 = 23.
// ```
//
// **Constraints:**
//
// *   The number of nodes in the tree is in the range `[1, 2 * 10<sup>4</sup>]`.
// *   `1 <= Node.val <= 10<sup>5</sup>`
// *   `1 <= low <= high <= 10<sup>5</sup>`
// *   All `Node.val` are **unique**.

// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//   pub val: i32,
//   pub left: Option<Rc<RefCell<TreeNode>>>,
//   pub right: Option<Rc<RefCell<TreeNode>>>,
// }
//
// impl TreeNode {
//   #[inline]
//   pub fn new(val: i32) -> Self {
//     TreeNode {
//       val,
//       left: None,
//       right: None
//     }
//   }
// }
use crate::base_tree::TreeNode;

use std::cell::RefCell;
use std::rc::Rc;

pub fn range_sum_bst(root: Option<Rc<RefCell<TreeNode>>>, low: i32, high: i32) -> i32 {
    if root.is_none() {
        return 0;
    }
    let mut sum = 0;
    let val = root.as_ref().unwrap().borrow().val;
    if val >= low && val <= high {
        sum = sum + val;
    }
    sum = sum + range_sum_bst(root.as_ref().unwrap().borrow().left.clone(), low, high);
    sum = sum + range_sum_bst(root.as_ref().unwrap().borrow().right.clone(), low, high);
    return sum;
}

#[test]
pub fn t1() {
    let mut root = TreeNode::new(10);
    root.left = Some(Rc::new(RefCell::new(TreeNode::new(5))));
    root.right = Some(Rc::new(RefCell::new(TreeNode::new(15))));
    root.left.as_ref().unwrap().borrow_mut().left = Some(Rc::new(RefCell::new(TreeNode::new(3))));
    root.left.as_ref().unwrap().borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode::new(7))));
    root.right.as_ref().unwrap().borrow_mut().right =
        Some(Rc::new(RefCell::new(TreeNode::new(18))));
    assert_eq!(range_sum_bst(Some(Rc::new(RefCell::new(root))), 7, 15), 32);
}
