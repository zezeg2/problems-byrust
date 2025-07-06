use crate::Solution;
use crate::dfs::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn max_path_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn helper(node: Option<Rc<RefCell<TreeNode>>>, max_sum: &mut i32) -> i32 {
            if let Some(n) = node {
                let n = n.borrow();
                let left = helper(n.left.clone(), max_sum).max(0); // 음수면 무시
                let right = helper(n.right.clone(), max_sum).max(0);

                let local_sum = n.val + left + right;
                *max_sum = (*max_sum).max(local_sum);

                n.val + left.max(right)
            } else {
                0
            }
        }

        let mut max_sum = i32::MIN;
        helper(root, &mut max_sum);
        max_sum
    }
}
