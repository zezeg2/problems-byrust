use crate::Solution;

use crate::dfs::TreeNode;
use std::cell::{Ref, RefCell};
use std::rc::Rc;
impl Solution {
    pub fn path_sum_iii(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> i32 {
        Solver::new(root, target_sum).solve()
    }
}
impl Solution {
    pub fn path_sum_iii_2(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> i32 {
        let mut solver = Solver2::new(target_sum);
        let mut path = vec![];
        solver.dfs_stack(root, &mut path);
        solver.count
    }
}

struct Solver {
    root: Option<Rc<RefCell<TreeNode>>>,
    target_sum: i64,
    count: i32,
}

impl Solver {
    fn new(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> Self {
        Solver {
            root,
            target_sum: target_sum as i64,
            count: 0,
        }
    }

    fn dfs(&mut self, node: Option<Rc<RefCell<TreeNode>>>) -> Vec<i64> {
        if let Some(n) = node {
            let n_clone = n.clone();
            let c_node = n_clone.borrow();
            let l_node = c_node.left.clone();
            let r_node = c_node.right.clone();

            let mut result = vec![];
            let l_candidates = self.dfs(l_node);
            let r_candidates = self.dfs(r_node);

            let l_added: Vec<i64> = l_candidates.iter().map(|i| i + c_node.val as i64).collect();
            let r_added: Vec<i64> = r_candidates.iter().map(|i| i + c_node.val as i64).collect();

            result.extend(l_added);
            result.extend(r_added);
            result.push(c_node.val as i64);

            result.iter().for_each(|i| {
                if *i == self.target_sum {
                    self.count += 1;
                }
            });
            result
        } else {
            vec![]
        }
    }

    fn solve(&mut self) -> i32 {
        self.dfs(self.root.clone());
        self.count
    }
}

// Alternative DFS using a path stack to accumulate sums, avoiding returning vectors.
struct Solver2 {
    target_sum: i64,
    count: i32,
}

impl Solver2 {
    fn new(target_sum: i32) -> Self {
        Solver2 {
            target_sum: target_sum as i64,
            count: 0,
        }
    }

    fn dfs_stack(&mut self, node: Option<Rc<RefCell<TreeNode>>>, path: &mut Vec<i64>) {
        if let Some(n) = node {
            let n = n.borrow();
            path.push(n.val as i64);

            let mut sum = 0;
            for &v in path.iter().rev() {
                sum += v;
                if sum == self.target_sum {
                    self.count += 1;
                }
            }

            self.dfs_stack(n.left.clone(), path);
            self.dfs_stack(n.right.clone(), path);

            path.pop();
        }
    }
}

fn new_node(val: i32) -> Rc<RefCell<TreeNode>> {
    Rc::new(RefCell::new(TreeNode {
        val,
        left: None,
        right: None,
    }))
}

fn insert_left(parent: &Rc<RefCell<TreeNode>>, child: Rc<RefCell<TreeNode>>) {
    parent.borrow_mut().left = Some(child);
}

fn insert_right(parent: &Rc<RefCell<TreeNode>>, child: Rc<RefCell<TreeNode>>) {
    parent.borrow_mut().right = Some(child);
}

#[test]
fn test_path_sum_iii_example1() {
    use crate::Solution;

    let root = new_node(10);
    let node5 = new_node(5);
    let node3 = new_node(3);
    let node3_leaf = new_node(3);
    let node_minus2 = new_node(-2);
    let node2 = new_node(2);
    let node1 = new_node(1);
    let node_minus3 = new_node(-3);
    let node11 = new_node(11);

    insert_left(&root, node5.clone());
    insert_right(&root, node_minus3.clone());

    insert_left(&node5, node3.clone());
    insert_right(&node5, node2.clone());

    insert_left(&node3, node3_leaf.clone());
    insert_right(&node3, node_minus2.clone());

    insert_right(&node2, node1.clone());

    insert_right(&node_minus3, node11.clone());

    let count = Solution::path_sum_iii(Some(root), 8);
    assert_eq!(count, 3); // Expected output from example
}
