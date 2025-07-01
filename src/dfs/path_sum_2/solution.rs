use crate::Solution;
use std::cell::RefCell;
use std::rc::Rc;
use crate::dfs::TreeNode;

impl Solution {
    pub fn path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> Vec<Vec<i32>> {
        fn search (root: Option<Rc<RefCell<TreeNode>>>, path_vec: &mut Vec<i32>, target_sum: i32, result: &mut Vec<Vec<i32>>) {
            if let Some(node) = root {
                let node = node.borrow();
                path_vec.push(node.val);
                if node.left.is_none() && node.right.is_none() {
                    let sum: i32 = path_vec.iter().sum();
                    if sum == target_sum {
                        result.push(path_vec.clone())
                    }
                } else {
                    search(node.left.clone(), path_vec, target_sum, result);
                    search(node.right.clone(), path_vec, target_sum, result);
                }
                path_vec.pop();
            }
        }

        let mut result: Vec<Vec<i32>> = vec![];
        search(root, &mut vec![], target_sum, &mut result);
        result
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    fn build_example_tree() -> Option<Rc<RefCell<TreeNode>>> {
        let n7 = Rc::new(RefCell::new(TreeNode::new(7)));
        let n2 = Rc::new(RefCell::new(TreeNode::new(2)));
        let n5_leaf = Rc::new(RefCell::new(TreeNode::new(5)));
        let n1 = Rc::new(RefCell::new(TreeNode::new(1)));
        let n13 = Rc::new(RefCell::new(TreeNode::new(13)));

        let n11 = Rc::new(RefCell::new(TreeNode::new(11)));
        n11.borrow_mut().left = Some(n7.clone());
        n11.borrow_mut().right = Some(n2.clone());

        let n4_left = Rc::new(RefCell::new(TreeNode::new(4)));
        n4_left.borrow_mut().left = Some(n11.clone());

        let n4_right = Rc::new(RefCell::new(TreeNode::new(4)));
        n4_right.borrow_mut().left = Some(n5_leaf.clone());
        n4_right.borrow_mut().right = Some(n1.clone());

        let n8 = Rc::new(RefCell::new(TreeNode::new(8)));
        n8.borrow_mut().left = Some(n13.clone());
        n8.borrow_mut().right = Some(n4_right.clone());

        let root = Rc::new(RefCell::new(TreeNode::new(5)));
        root.borrow_mut().left = Some(n4_left.clone());
        root.borrow_mut().right = Some(n8.clone());

        Some(root)
    }

    #[test]
    fn test_path_sum_example_1() {
        let root = build_example_tree();
        let target_sum = 22;

        let mut result = Solution::path_sum(root, target_sum);
        let mut expected = vec![
            vec![5, 4, 11, 2],
            vec![5, 8, 4, 5],
        ];

        // 정렬해서 순서에 상관없이 비교
        result.sort();
        expected.sort();
        assert_eq!(result, expected);
    }
}