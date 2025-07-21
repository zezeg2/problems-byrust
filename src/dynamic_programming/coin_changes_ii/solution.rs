use crate::Solution;

impl Solution {
    pub fn change(amount: i32, coins: Vec<i32>) -> i32 {
        let mut dp = vec![0; amount as usize + 1];
        dp[0] = 1;

        for &coin in &coins {
            for x in coin..=amount {
                dp[x as usize] += dp[(x - coin) as usize];
            }
        }

        dp[amount as usize]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(Solution::change(5, vec![1, 2, 5]), 4);
    }

    #[test]
    fn case2() {
        assert_eq!(Solution::change(20, vec![3, 5, 11]), 20);
    }

    #[test]
    fn case3() {
        assert_eq!(Solution::change(10, vec![10]), 1);
    }
}
