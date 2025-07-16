use std::rc::Rc;
use std::cell::RefCell;
use crate::dfs::TreeNode;
use crate::Solution;


impl Solution {
    pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut result = vec![];
        Self::search_inorder(root, &mut result);
        result
    }

    fn search_inorder(root: Option<Rc<RefCell<TreeNode>>>, vec: &mut Vec<i32>) {
        match root {
            None => {}
            Some(node) => {
                let inner = node.borrow();
                Self::search_inorder(inner.left.clone(), vec);
                vec.push(inner.val);
                Self::search_inorder(inner.right.clone(), vec);
            }
        }
    }
}