use crate::Solution;
use std::cell::RefCell;
use std::rc::Rc;
use crate::dfs::TreeNode;

impl Solution {
    pub fn is_same_tree(
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
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
