use crate::Solution;

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let days = prices.len();
        let mut dp: Vec<i32> = vec![0; days];

        for d in 1..days {
            let diff = prices[d] - prices[d - 1];
            dp[d] = dp[d - 1] + diff.max(0);
        }

        dp[days-1]
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(Solution::max_profit(vec![7,1,5,3,6,4]), 7);
    }
}