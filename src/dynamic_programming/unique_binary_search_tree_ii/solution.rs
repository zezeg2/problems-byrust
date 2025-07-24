use crate::Solution;

impl Solution {
    pub fn num_trees(n: i32) -> i32 {
        let n = n as usize;
        let  mut dp = vec![0; n + 1];
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