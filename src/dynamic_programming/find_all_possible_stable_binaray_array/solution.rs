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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(1, Solution::number_of_stable_arrays(3,3,2))
    }
}