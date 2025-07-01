use std::rc::Rc;
use std::cell::RefCell;
use crate::dfs::TreeNode;
use crate::Solution;

impl Solution {
    pub fn recover_tree(root: &Option<Rc<RefCell<TreeNode>>>) {
        // 중위 순회 중 상태를 저장할 변수들
        let mut prev: Option<Rc<RefCell<TreeNode>>> = None;
        let mut first: Option<Rc<RefCell<TreeNode>>> = None;
        let mut second: Option<Rc<RefCell<TreeNode>>> = None;

        // 내부 중위 순회 함수 정의 (재귀)
        fn inorder(
            node: &Option<Rc<RefCell<TreeNode>>>,
            prev: &mut Option<Rc<RefCell<TreeNode>>>,
            first: &mut Option<Rc<RefCell<TreeNode>>>,
            second: &mut Option<Rc<RefCell<TreeNode>>>,
        ) {
            if let Some(current) = node {
                let current = current.clone(); // Rc 복제
                let left = current.borrow().left.clone(); // 왼쪽 자식 가져오기
                inorder(&left, prev, first, second); // 왼쪽 서브트리 탐색

                // 현재 노드 처리
                if let Some(prev_node) = prev {
                    if prev_node.borrow().val > current.borrow().val {
                        // BST 성질을 위반하는 경우 발견
                        if first.is_none() {
                            // 첫 번째 노드 저장 (처음 위반 발견 시)
                            *first = Some(prev_node.clone());
                        }
                        // 두 번째 노드는 계속 업데이트 가능
                        *second = Some(current.clone());
                    }
                }

                // prev 업데이트
                *prev = Some(current.clone());

                let right = current.borrow().right.clone(); // 오른쪽 자식 가져오기
                inorder(&right, prev, first, second); // 오른쪽 서브트리 탐색
            }
        }

        // 중위 순회로 first, second 찾기
        inorder(root, &mut prev, &mut first, &mut second);

        // 두 노드의 값을 스왑
        if let (Some(f), Some(s)) = (first, second) {
            let mut f_val = f.borrow_mut();
            let mut s_val = s.borrow_mut();
            std::mem::swap(&mut f_val.val, &mut s_val.val);
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::rc::Rc;
    use std::cell::RefCell;

    fn to_inorder_values(root: &Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut result = vec![];

        fn inorder(node: &Option<Rc<RefCell<TreeNode>>>, out: &mut Vec<i32>) {
            if let Some(rc) = node {
                let node = rc.borrow();
                inorder(&node.left, out);
                out.push(node.val);
                inorder(&node.right, out);
            }
        }

        inorder(root, &mut result);
        result
    }

    #[test]
    fn test_recover_tree_disjoint_swapped_nodes() {
        // Tree before recovery (inorder: 6 3 4 5 2):
        //        5
        //       / \
        //      3   6
        //     /     \
        //    2       4
        //
        // 6 and 2 are swapped

        let n2 = Rc::new(RefCell::new(TreeNode::new(2))); // should be 6
        let n4 = Rc::new(RefCell::new(TreeNode::new(4)));
        let n3 = Rc::new(RefCell::new(TreeNode::new(3)));
        let n6 = Rc::new(RefCell::new(TreeNode::new(6))); // should be 2
        let n5 = Rc::new(RefCell::new(TreeNode::new(5)));

        n3.borrow_mut().left = Some(n6.clone());  // wrong: left of 3 is 6 (should be 2)
        n6.borrow_mut().right = Some(n4.clone()); // right of 6 is 4
        n5.borrow_mut().left = Some(n3.clone());  // 5.left = 3
        n5.borrow_mut().right = Some(n2.clone()); // wrong: right of 5 is 2 (should be 6)

        let root = Some(n5.clone());

        // Before recovery: inorder = [6, 3, 4, 5, 2]
        let before = to_inorder_values(&root);
        assert_eq!(before, vec![6, 3, 4, 5, 2]);

        Solution::recover_tree(&root);

        // After recovery: inorder = [2, 3, 4, 5, 6]
        let after = to_inorder_values(&root);
        assert_eq!(after, vec![2, 3, 4, 5, 6]);
    }
}