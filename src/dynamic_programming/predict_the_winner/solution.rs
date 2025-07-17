use crate::Solution;

use std::collections::HashMap;

impl Solution {
    pub fn predict_the_winner(nums: Vec<i32>) -> bool {
        fn helper(nums: &[i32], left: usize, right: usize, memo: &mut HashMap<(usize, usize), i32>) -> i32 {
            if left == right {
                return nums[left];
            }

            if let Some(&val) = memo.get(&(left, right)) {
                return val;
            }

            // 현재 플레이어가 양 끝 중 하나 선택
            let pick_left = nums[left] - helper(nums, left + 1, right, memo);
            let pick_right = nums[right] - helper(nums, left, right - 1, memo);

            let res = pick_left.max(pick_right);
            memo.insert((left, right), res);
            res
        }

        let mut memo = HashMap::new();
        helper(&nums, 0, nums.len() - 1, &mut memo) >= 0
    }
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn case1() {
        assert_eq!(Solution::predict_the_winner(vec![1,5,2]), true)
    }
}