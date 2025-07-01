use crate::Solution;
use std::cell::RefCell;
use std::rc::Rc;
use crate::dfs::TreeNode;

impl Solution {
    pub fn has_path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> bool {
        if let Some(node) = root {
            let node = node.borrow();
            if node.left.is_none() && node.right.is_none() {
                return node.val == target_sum;
            }
            Self::has_path_sum(node.left.clone(), target_sum - node.val)
                || Self::has_path_sum(node.right.clone(), target_sum - node.val)
        } else {
            false
        }
    }
}
