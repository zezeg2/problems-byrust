use std::rc::Rc;
use std::cell::RefCell;
use crate::dfs::TreeNode;
use crate::Solution;

impl Solution {
    pub fn flatten(root: &mut Option<Rc<RefCell<TreeNode>>>) {
        fn preorder(
            node: &Option<Rc<RefCell<TreeNode>>>,
            prev: &mut Option<Rc<RefCell<TreeNode>>>,
        ) {
            if let Some(n) = node {
                let current = Rc::clone(n);

                // 왼쪽, 오른쪽 자식도 미리 clone()해서 재귀 순서를 보존
                let left = current.borrow().left.clone();
                let right = current.borrow().right.clone();

                // 이전 노드가 있다면, 그 노드의 오른쪽을 현재 노드로 연결
                if let Some(prev_node) = prev {
                    prev_node.borrow_mut().right = Some(current.clone());
                    prev_node.borrow_mut().left = None;
                }

                // 현재 노드를 prev로 설정
                *prev = Some(current);

                // 왼쪽 -> 오른쪽 순으로 순회
                preorder(&left, prev);
                preorder(&right, prev);
            }
        }

        let mut prev: Option<Rc<RefCell<TreeNode>>> = None;
        preorder(root, &mut prev);
    }

    pub fn flatten_reverse_preorder(root: &mut Option<Rc<RefCell<TreeNode>>>) {
        fn helper(node: Option<Rc<RefCell<TreeNode>>>, prev: &mut Option<Rc<RefCell<TreeNode>>>) {
            if let Some(n) = node {
                let right = n.borrow_mut().right.clone();
                let left = n.borrow_mut().left.clone();

                helper(right, prev);
                helper(left, prev);

                n.borrow_mut().right = prev.clone();
                n.borrow_mut().left = None;
                *prev = Some(n.clone());
            }
        }

        let mut prev = None;
        helper(root.clone(), &mut prev);
    }
}