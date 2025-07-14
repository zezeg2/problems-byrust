use crate::Solution;

impl Solution {
    pub fn unique_paths_with_obstacles(obstacle_grid: Vec<Vec<i32>>) -> i32 {
        let r = obstacle_grid.len();
        let c = obstacle_grid[0].len();

        let get_mxn = |m: usize, n: usize| -> i32 { obstacle_grid[m][n] };

        let mut dp = vec![0; c];

        for i in 0..r {
            for j in 0..c {
                if get_mxn(i, j) == 1 {
                    dp[j] = 0
                } else {
                    dp[j] = match (i, j) {
                        (0, 0) => 1,
                        (_, 0) => dp[j],
                        (0, _) => dp[j - 1],
                        _ => dp[j] + dp[j - 1],
                    }
                }
            }
        }

        dp[c - 1]
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn case1() {
        assert_eq!(
            Solution::unique_paths_with_obstacles(vec![
                vec![0, 0, 0],
                vec![0, 1, 0],
                vec![0, 0, 0]
            ]),
            2
        )
    }
}
