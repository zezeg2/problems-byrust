use crate::Solution;

impl Solution {
    pub fn number_of_arithmetic_slices(nums: Vec<i32>) -> i32 {
        let len = nums.len();
        let mut dp = vec![0; len];
        if len < 3 {
            0
        } else {
            let mut diff = nums[1] - nums[0];
            let mut seq_len = 0;
            for i in 1..len {
                if diff != nums[i] - nums[i - 1] {
                    diff = nums[i] - nums[i - 1];
                    seq_len = 2;
                } else {
                    seq_len += 1;
                }
                dp[i] = dp[i - 1];
                if seq_len >= 3 {
                    dp[i] +=  seq_len - 2;
                }
            }
            dp[len - 1]
        }
    }
}


#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn mycase(){
        assert_eq!(Solution::number_of_arithmetic_slices(vec![1,2,3,4,5,7,9,-11,14,123,7,9,11]),8)
    }

    #[test]
    fn case1(){
        assert_eq!(Solution::number_of_arithmetic_slices(vec![1,2,3,4]),3)
    }

    #[test]
    fn case2(){
        assert_eq!(Solution::number_of_arithmetic_slices(vec![1]),0)
    }
}