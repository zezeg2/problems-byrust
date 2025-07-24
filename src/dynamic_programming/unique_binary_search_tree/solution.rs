use crate::Solution;
use crate::dfs::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn generate_trees(n: i32) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
        if n == 0 {
            return vec![];
        }

        fn build_trees(start: i32, end: i32) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
            let mut all_trees = Vec::new();
            if start > end {
                all_trees.push(None);
                return all_trees;
            }

            for i in start..=end {
                // 왼쪽 서브트리 구성
                let left_trees = build_trees(start, i - 1);
                // 오른쪽 서브트리 구성
                let right_trees = build_trees(i + 1, end);

                for left in &left_trees {
                    for right in &right_trees {
                        let node = Rc::new(RefCell::new(TreeNode {
                            val: i,
                            left: left.clone(),
                            right: right.clone(),
                        }));
                        all_trees.push(Some(node));
                    }
                }
            }

            all_trees
        }

        build_trees(1, n)
    }
    pub fn num_trees_(n: i32) -> i32 {
        let n = n as usize;
        let mut dp = vec![0; n + 1];
        dp[0] = 1;

        for i in 1..=n {
            for j in 0..i {
                dp[i] += dp[j] * dp[i - 1 - j];
            }
        }
        dp[n]
    }
}


#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn case1() {
        assert_eq!(Solution::num_trees(3), 5);
    }
}
