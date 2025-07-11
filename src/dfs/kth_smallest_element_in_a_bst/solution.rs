use crate::Solution;
use crate::dfs::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn kth_smallest(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> i32 {
        Solver::new(k).solve(root)
    }
}

struct Solver {
    st: Vec<i32>,
    k: i32,
}

impl Solver {
    pub fn new(k: i32) -> Self {
        Solver { k, st: vec![] }
    }

    pub fn solve(&mut self, root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        self.dfs(root);
        let vec = &mut self.st;
        vec.sort_by(|a, b| a.cmp(b));
        *vec.get(self.k as usize - 1).unwrap()
    }

    pub fn dfs(&mut self, node: Option<Rc<RefCell<TreeNode>>>) {
        if let Some(n) = node {
            self.dfs(n.borrow().left.clone());
            self.st.push(n.borrow().val);
            self.dfs(n.borrow().right.clone());
        }
    }
}
