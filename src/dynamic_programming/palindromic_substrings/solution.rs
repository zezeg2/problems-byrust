
use std::collections::HashMap;
use crate::Solution;

impl Solution {
    pub fn count_substrings(s: String) -> i32 {
        let n = s.len();
        let s = s.as_bytes();
        let mut dp = vec![vec![false; n]; n];
        let mut count = 0;

        for len in 1..=n {
            for i in 0..=n - len {
                let j = i + len - 1;

                if s[i] == s[j] && (len <= 2 || dp[i + 1][j - 1]) {
                    dp[i][j] = true;
                    count += 1;
                }
            }
        }

        count
    }

    pub fn count_substrings_not_good(s: String) -> i32 {
        let mut memo: HashMap<&str, bool> = HashMap::new();
        let mut dp: Vec<i32> = vec![0; s.len() + 1];

        for i in 1..=s.len() {
            dp[i] = dp[i-1] + (0..i).map(|j| {
                let sub = &s[j..i];
                if let Some(b) = memo.get(sub) {
                    return if *b { 1 } else { 0 }
                } else {
                    let is_palindrome = Self::is_palindrome(sub);
                    memo.insert(sub, is_palindrome);
                    return if is_palindrome { 1 } else { 0 }
                }
            }).sum::<i32>()
        }
        dp[s.len()]
    }

    fn is_palindrome(s: &str) -> bool {
        let bytes = s.as_bytes();
        let mut left = 0;
        let mut right = bytes.len() - 1;

        while left < right {
            if bytes[left] != bytes[right] {
                return false;
            }
            left += 1;
            right -= 1;
        }
        true
    }

    pub fn count_substrings_center_expand(s: String) -> i32 {
        let s = s.as_bytes();
        let n = s.len();
        let mut count = 0;

        for center in 0..2 * n - 1 {
            let mut left = center / 2;
            let mut right = left + center % 2;

            while left < n && right < n && s[left] == s[right] {
                count += 1;
                if left == 0 { break; }
                left -= 1;
                right += 1;
            }
        }

        count
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn case1() {
        assert_eq!(Solution::count_substrings("abc".to_string()),3)
    }
}