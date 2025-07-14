use std::collections::HashMap;
use crate::Solution;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut map = HashMap::new(); // char → 최근 위치
        let (mut left, mut max_len) = (0, 0);

        for (right, c) in s.chars().enumerate() {
            if let Some(&prev_index) = map.get(&c) {
                // left는 뒤로 가지 않게 max로 고정
                left = left.max(prev_index + 1);
            }
            map.insert(c, right);
            max_len = max_len.max(right - left + 1);
        }

        max_len as i32
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_single_space() {
        let input = "abxyzkwyw".to_string();
        let result = Solution::length_of_longest_substring(input);
        assert_eq!(result, 3); // 공백도 하나의 문자이므로 길이 1이 되어야 함
    }
}