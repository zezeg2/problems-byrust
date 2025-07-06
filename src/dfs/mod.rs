use std::cell::RefCell;
use std::rc::Rc;

mod binary_tree_inorder_traversal;
mod recovery_binary_search_tree;
mod same_tree;
mod path_sum;
mod path_sum_2;
mod symmetric_tree;
mod flatten_binarytree_to_linkedlist;
mod max_min_depth;
mod course_schedule;
mod binary_tree_maximum_path_sum;
mod course_schedule_2;
mod path_sum_3;

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