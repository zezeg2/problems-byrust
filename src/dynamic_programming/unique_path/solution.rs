use crate::Solution;

impl Solution {
    pub fn unique_paths(m: i32, n: i32) -> i32 {
        let (m, n) = (m as usize, n as usize);
        let mut dp = vec![1; n]; // 초기값: 첫 행은 모두 1

        for _ in 1..m {
            // 두 번째 행부터 시작
            for j in 1..n {
                dp[j] += dp[j - 1]; // 현재 셀 = 왼쪽 + 위 (위는 이전 dp[j], 왼쪽은 dp[j-1])
            }
        }

        dp[n - 1] // 맨 오른쪽 아래가 최종 결과
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn case1() {
        let m = 13;
        let n = 23;
        assert_eq!(Solution::unique_paths(m, n), 548354040)
    }
}
