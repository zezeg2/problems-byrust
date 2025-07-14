# 3129. Find All Possible Stable Binary Arrays

![3129 Find All Possible Stable Binary Arrays I.png](../../../images/3129%20Find%20All%20Possible%20Stable%20Binary%20Arrays%20I.png)

## Constraints

* `1 <= zero, one, limit <= 200`

## Solution

This problem can be solved using dynamic programming. We can define `dp0[i][j]` as the number of stable arrays with `i` zeros and `j` ones ending with 0, and `dp1[i][j]` as the number of stable arrays with `i` zeros and `j` ones ending with 1.

The state transitions are as follows:

* To form a new stable array ending in 0 with `i` zeros and `j` ones, we can append a 0 to any stable array with `i-1` zeros and `j` ones. The number of ways to do this is `(dp0[i-1][j] + dp1[i-1][j])`. However, we must subtract the cases where adding a 0 violates the `limit` condition. This occurs if the last `limit` elements were all 0s. The number of such cases is `dp1[i-limit-1][j]`.

* Similarly, to form a new stable array ending in 1 with `i` zeros and `j` ones, we can append a 1 to any stable array with `i` zeros and `j-1` ones. The number of ways is `(dp0[i][j-1] + dp1[i][j-1])`. We must subtract the cases where adding a 1 violates the `limit` condition, which is `dp0[i][j-limit-1]`.

```rust
use crate::Solution;
impl Solution {
    pub fn number_of_stable_arrays(zero: i32, one: i32, limit: i32) -> i32 {
        let zero = zero as usize;
        let one = one as usize;
        let limit = limit as usize;
        let modulo = 1_000_000_007;

        let mut dp0 = vec![vec![0; one + 1]; zero + 1];
        let mut dp1 = vec![vec![0; one + 1]; zero + 1];

        for i in 1..=zero.min(limit) { dp0[i][0] = 1; }
        for j in 1..=one.min(limit) { dp1[0][j] = 1; }

        for i in 1..=zero {
            for j in 1..=one {
                let ways_to_add_zero = (dp1[i - 1][j] + dp0[i - 1][j]) % modulo;
                if i > limit {
                    let subtract = dp1[i - limit - 1][j];
                    dp0[i][j] = (ways_to_add_zero - subtract + modulo) % modulo;
                } else {
                    dp0[i][j] = ways_to_add_zero;
                }

                let ways_to_add_one = (dp0[i][j - 1] + dp1[i][j - 1]) % modulo;
                if j > limit {
                    let subtract = dp0[i][j - limit - 1];
                    dp1[i][j] = (ways_to_add_one - subtract + modulo) % modulo;
                } else {
                    dp1[i][j] = ways_to_add_one;
                }
            }
        }

        (dp0[zero][one] + dp1[zero][one]) % modulo
    }
}
```
