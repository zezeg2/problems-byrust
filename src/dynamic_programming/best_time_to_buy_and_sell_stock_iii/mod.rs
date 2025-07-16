use crate::Solution;

impl Solution {
    pub fn max_profit_iii(prices: Vec<i32>) -> i32 {
        let days = prices.len();
        if days == 0 {
            return 0;
        }

        // dp[day][k]: day일까지 k번 거래했을 때의 최대 이익
        let mut dp = vec![vec![0; 3]; days];

        for k in 1..=2 {
            // max_diff: 이전 날까지의 (이익 - 주가) 최대값
            let mut max_diff = -prices[0];
            for day in 1..days {
                dp[day][k] = dp[day - 1][k].max(prices[day] + max_diff);
                max_diff = max_diff.max(dp[day][k - 1] - prices[day]);
            }
        }

        dp[days - 1][2]
    }

    pub fn max_profit_iii_an(prices: Vec<i32>) -> i32 {
        let mut hold1 = i32::MIN;
        let mut release1 = 0;
        let mut hold2 = i32::MIN;
        let mut release2 = 0;

        for price in prices {
            release2 = release2.max(hold2 + price);
            hold2 = hold2.max(release1 - price);
            release1 = release1.max(hold1 + price);
            hold1 = hold1.max(-price);
        }

        release2
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(Solution::max_profit_iii(vec![3, 3, 5, 0, 0, 3, 1, 4]), 6);
    }
}
