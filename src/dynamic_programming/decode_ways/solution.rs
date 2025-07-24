use std::collections::HashMap;
use crate::Solution;

impl Solution {
    pub fn num_decodings_dp(s: String) -> i32 {
        let n = s.len();
        let bytes = s.as_bytes();
        let mut dp = vec![0; n + 1];
        dp[n] = 1; // base case

        for i in (0..n).rev() {
            if bytes[i] == b'0' {
                dp[i] = 0;
            } else {
                dp[i] = dp[i + 1]; // 한 글자
                if i + 1 < n {
                    let num = (bytes[i] - b'0') * 10 + (bytes[i + 1] - b'0');
                    if num <= 26 {
                        dp[i] += dp[i + 2]; // 두 글자
                    }
                }
            }
        }

        dp[0]
    }
    pub fn num_decodings(s: String) -> i32 {
        fn dfs(s: &str, i: usize, memo: &mut HashMap<usize, i32>) -> i32 {
            if i == s.len() {
                return 1;
            }
            if s.as_bytes()[i] == b'0' {
                return 0;
            }
            if let Some(&val) = memo.get(&i) {
                return val;
            }

            let mut count = dfs(s, i + 1, memo);
            if i + 1 < s.len() {
                let two = &s[i..=i + 1];
                if two <= "26" {
                    count += dfs(s, i + 2, memo);
                }
            }

            memo.insert(i, count);
            count
        }

        let mut memo = HashMap::new();
        dfs(&s, 0, &mut memo)
    }

    pub fn num_decodings_time_limit_exceed(s: String) -> i32 {
        fn helper(left: String, start: usize, cnt: &mut i32) -> bool {
            if left.len() == 0 {
                *cnt += 1;
                true
            } else {
                let c1 = &left[0..1];

                if c1.parse::<i32>().unwrap() == 0 {
                    return false;
                }

                helper(left[1..].to_string(), start + 1, cnt);

                if left.len() >= 2 {
                    let c2 = &left[0..2];
                    let c2_i: i32 = c2.parse().unwrap();

                    if c2_i < 27 {
                        helper(left[2..].to_string(), start + 2, cnt);
                    }
                }
                true
            }
        }

        let mut cnt = 0;

        helper(s, 1, &mut cnt);
        cnt
    }
}

#[cfg(test)]
mod tests {
    use std::collections::{BinaryHeap, LinkedList};
    use super::*;

    #[test]
    fn test_num_decodings() {
        assert_eq!(Solution::num_decodings("06".to_string()), 0);
    }
}
