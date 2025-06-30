#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
  pub val: i32,
  pub left: Option<Rc<RefCell<TreeNode>>>,
  pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
  #[inline]
  pub fn new(val: i32) -> Self {
    TreeNode {
      val,
      left: None,
      right: None
    }
  }
}

use std::rc::Rc;
use std::cell::RefCell;
use crate::Solution;

impl Solution {
    pub fn is_same_tree(p: Option<Rc<RefCell<TreeNode>>>, q: Option<Rc<RefCell<TreeNode>>>) -> bool {
        match (p, q) {
            (None, None) => true,
            (Some(p_node), Some(q_node)) => {
                let p_node = p_node.borrow();
                let q_node = q_node.borrow();
                p_node.val == q_node.val
                    && Self::is_same_tree(p_node.left.clone(), q_node.left.clone())
                    && Self::is_same_tree(p_node.right.clone(), q_node.right.clone())
            }
            _ => false, // One is Some, the other is None
        }
    }
}