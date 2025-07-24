use crate::Solution;

impl Solution {
    pub fn num_perms_di_sequence(s: String) -> i32 {
        let n = s.len();
        let modulo = 1_000_000_007;
        let mut dp = vec![vec![0; n + 1]; n + 1];

        dp[0][0] = 1;
        println!("Step 0: dp[0] = {:?}", dp[0]);

        for i in 1..=n {
            let ch = s.as_bytes()[i - 1];
            let mut prefix_sum = vec![0; n + 2];
            for j in 0..=i {
                prefix_sum[j + 1] = (prefix_sum[j] + dp[i - 1][j]) % modulo;
            }

            println!("\nStep {}: processing '{}'", i, ch as char);
            println!("  dp[{} - 1] = {:?}", i, dp[i - 1]);
            println!("  prefix_sum = {:?}", &prefix_sum[..=i+1]);

            for j in 0..=i {
                if ch == b'I' {
                    dp[i][j] = prefix_sum[j];
                } else {
                    dp[i][j] = (prefix_sum[i] + modulo - prefix_sum[j]) % modulo;
                }
                println!("    dp[{}][{}] = {}", i, j, dp[i][j]);
            }
        }

        println!("\nFinal dp[n] = {:?}", dp[n]);
        dp[n].iter().fold(0, |acc, &x| (acc + x) % modulo)
    }

    pub fn num_perms_di_sequence_another(s: String) -> i32 {
        const MOD: i64 = 1_000_000_007;

        let n = s.len();

        // 초기 상태: 0번째 단계에서는 단 하나의 방법만 존재 (perm[0] = 0)
        let mut prev = vec![1; n + 1];

        // s의 각 문자 (즉, 각 비교 지점)를 순회
        for (i, ch) in s.chars().enumerate() {
            let n = n - i; // 현재 가능한 자리 수는 점점 줄어든다 (n+1 → n)

            // 이번 단계의 결과를 담을 dp 테이블
            let mut dp = vec![0; n];

            // 이전 단계의 값을 기반으로 새로운 dp를 채운다
            for (above, count) in prev.into_iter().enumerate() {
                // ch가 'D'이면 현재 값이 이전 값보다 작아야 하므로
                // 현재 위치에 올 수 있는 수들은 [above..n)
                // ch가 'I'이면 현재 값이 이전 값보다 커야 하므로
                // 올 수 있는 수들은 [0..above)
                let range = if ch == 'D' {
                    above..n
                } else {
                    0..above
                };

                // 해당 range에 count를 누적
                for j in range {
                    dp[j] += count;
                    dp[j] %= MOD;
                }
            }

            // 다음 반복을 위해 prev를 갱신
            prev = dp;
        }

        // 마지막에는 유효한 순열이 prev[0]에만 저장되어 있음
        prev[0] as i32
    }
}


#[cfg(test)]
mod tes {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(Solution::num_perms_di_sequence("DIDID".to_string()),5)
    }

    #[test]
    fn trace_example() {
        const MOD: i64 = 1_000_000_007;

        let s = "ID".to_string(); // 예제 입력
        let n = s.len();
        let mut prev = vec![1; n + 1];
        println!("Initial prev: {:?}", prev);

        for (i, ch) in s.chars().enumerate() {
            let step = i + 1;
            let mut dp = vec![0; n + 1 - i];
            let curr_len = dp.len();
            println!("\nStep {} - processing '{}'", step, ch);

            for (above, count) in prev.into_iter().enumerate() {
                let range = if ch == 'D' {
                    above..curr_len
                } else {
                    0..above
                };

                for j in range.clone() {
                    dp[j] += count;
                    dp[j] %= MOD;
                }

                println!(
                    "  prev index {} (count = {}) → adds to dp[{:?}]",
                    above, count, range.collect::<Vec<_>>()
                );
            }

            println!("Resulting dp: {:?}", dp);
            prev = dp;
        }

        println!("\nFinal result: {}", prev[0]);
    }
}