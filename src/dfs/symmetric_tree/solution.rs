use crate::Solution;
use std::cell::RefCell;
use std::rc::Rc;
use crate::dfs::TreeNode;

impl Solution {
    pub fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        fn is_mirror(
            t1: Option<Rc<RefCell<TreeNode>>>,
            t2: Option<Rc<RefCell<TreeNode>>>,
        ) -> bool {
            match (t1, t2) {
                (None, None) => true,
                (Some(n1), Some(n2)) => {
                    let n1 = n1.borrow();
                    let n2 = n2.borrow();
                    n1.val == n2.val
                        && is_mirror(n1.left.clone(), n2.right.clone())
                        && is_mirror(n1.right.clone(), n2.left.clone())
                }
                _ => false,
            }
        }

        if let Some(node) = root {
            let node = node.borrow();
            is_mirror(node.left.clone(), node.right.clone())
        } else {
            true
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::rc::Rc;
    use std::cell::RefCell;

    #[test]
    fn test_symmetric_tree() {
        // 생성자 단축
        fn node(val: i32) -> Rc<RefCell<TreeNode>> {
            Rc::new(RefCell::new(TreeNode::new(val)))
        }

        // 하위 노드 구성
        let root = node(1);
        let left = node(2);
        let right = node(2);

        let left_left = node(3);
        let left_right = node(4);
        let right_left = node(4);
        let right_right = node(3);

        root.borrow_mut().left = Some(left.clone());
        root.borrow_mut().right = Some(right.clone());

        left.borrow_mut().left = Some(left_left);
        left.borrow_mut().right = Some(left_right);
        right.borrow_mut().left = Some(right_left);
        right.borrow_mut().right = Some(right_right);

        let result = Solution::is_symmetric(Some(root));
        assert!(result);
    }
}
