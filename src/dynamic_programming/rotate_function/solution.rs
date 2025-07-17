use crate::Solution;

impl Solution {
    pub fn max_rotate_function(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut dp: Vec<i32> = vec![0; n];
        let dp_0: i32 = nums
            .iter()
            .enumerate()
            .map(|(i, &val)| i as i32 * val)
            .sum();
        dp[0] = dp_0;

        let sum: i32 = nums.iter().sum();

        for i in 1..n {
            dp[i] = dp[i - 1] + sum - n as i32 * nums[n - i];
        }
        *dp.iter().max().unwrap()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(Solution::max_rotate_function(vec![4, 3, 2, 6]), 26)
    }

    #[test]
    fn case2() {
        assert_eq!(Solution::max_rotate_function(vec![100]), 0)
    }

    #[test]
    fn case3() {
        assert_eq!(Solution::max_rotate_function(vec![1,2,3,4,5,6,7,8,9,10]), 0)
    }
}
