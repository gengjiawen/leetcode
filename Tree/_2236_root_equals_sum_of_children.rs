// https://leetcode.com/problems/root-equals-sum-of-children
//
// You are given the `root` of a **binary tree** that consists of exactly `3` nodes: the root, its left child, and its right child.
//
// Return `true` _if the value of the root is equal to the **sum** of the values of its two children, or_ `false` _otherwise_.
//
// **Example 1:**
//
// ![](https://assets.leetcode.com/uploads/2022/04/08/graph3drawio.png)
// ```
// **Input:** root = [10,4,6]
// **Output:** true
// **Explanation:** The values of the root, its left child, and its right child are 10, 4, and 6, respectively.
// 10 is equal to 4 + 6, so we return true.
// ```
//
// **Example 2:**
//
// ![](https://assets.leetcode.com/uploads/2022/04/08/graph3drawio-1.png)
// ```
// **Input:** root = [5,3,1]
// **Output:** false
// **Explanation:** The values of the root, its left child, and its right child are 5, 3, and 1, respectively.
// 5 is not equal to 3 + 1, so we return false.
// ```
//
// **Constraints:**
//
// *   The tree consists only of the root, its left child, and its right child.
// *   `-100 <= Node.val <= 100`

use std::cell::RefCell;
use std::rc::Rc;
use crate::base_tree::TreeNode;

pub fn check_tree(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
    let root_node = root.as_ref().unwrap().borrow();
    let root_val = root.as_ref().unwrap().borrow().val;
    let left_val = root_node.left.as_ref().unwrap().borrow().val;
    let right_val = root_node.right.as_ref().unwrap().borrow().val;
    return root_val == left_val + right_val;
}

#[test]
pub fn t1() {
    let mut root = TreeNode::new(10);
    root.left = Some(Rc::new(RefCell::new(TreeNode::new(4))));
    root.right = Some(Rc::new(RefCell::new(TreeNode::new(6))));
    assert_eq!(check_tree(Some(Rc::new(RefCell::new(root)))), true);
}
