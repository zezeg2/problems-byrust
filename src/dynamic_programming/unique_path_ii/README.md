# Unique Paths II
![63 Unique Paths II.png](../../../images/63%20Unique%20Paths%20II.png)

## Constraints

* `m == obstacleGrid.length`
* `n == obstacleGrid[i].length`
* `1 <= m, n <= 100`
* `obstacleGrid[i][j]` is `0` or `1`.

## Solution

This problem can be solved using dynamic programming. We can use a 1D array `dp` to store the number of unique paths to each cell in the current row.

The state transition is as follows:
* If `obstacle_grid[i][j]` is 1, then `dp[j]` is 0.
* Otherwise, `dp[j] = dp[j] + dp[j-1]`.

The base cases are:
* `dp[0]` is 1 if `obstacle_grid[0][0]` is 0.
* For the first row, `dp[j] = dp[j-1]` if `obstacle_grid[0][j]` is 0.
* For the first column, `dp[j]` remains the same if `obstacle_grid[i][0]` is 0.

```rust
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
```