use crate::Solution;
use crate::dfs::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if let Some(node) = root {
            let rc = node.clone();
            let left = rc.borrow().left.clone();
            let right = rc.borrow().right.clone();
            return (1 + Self::max_depth(left)).max(1 + Self::max_depth(right));
        }
        0
    }

    pub fn min_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if let Some(node) = root {
            let node_ref = node.borrow();
            let left = node_ref.left.clone();
            let right = node_ref.right.clone();

            match (left, right) {
                (None, None) => 1,
                (Some(l), None) => 1 + Self::min_depth(Some(l)),
                (None, Some(r)) => 1 + Self::min_depth(Some(r)),
                (Some(l), Some(r)) => {
                    let d1 = Self::min_depth(Some(l));
                    let d2 = Self::min_depth(Some(r));
                    1 + d1.min(d2)
                }
            }
        } else {
            0
        }
    }
}
