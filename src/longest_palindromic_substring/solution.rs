use crate::Solution;
impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        // 1. Vec<char>로 변환
        let chars: Vec<char> = s.chars().collect();
        let n = chars.len();
        if n < 2 {
            return s; // 길이 < 2는 자기 자신이 팰린드롬
        }

        let mut start = 0;      // 최장 팰린드롬의 시작 인덱스
        let mut max_len = 1;    // 최장 팰린드롬의 길이

        for i in 0..n {
            // (1) 홀수 길이: center at i
            let (l1, r1) = expand(&chars, i, i);
            if r1 - l1 + 1 > max_len {
                start = l1;
                max_len = r1 - l1 + 1;
            }

            // (2) 짝수 길이: center between i and i+1
            if i + 1 < n && chars[i] == chars[i + 1]{
                let (l2, r2) = expand(&chars, i, i + 1);
                if r2 - l2 + 1 > max_len {
                    start = l2;
                    max_len = r2 - l2 + 1;
                }
            }
        }

        // char 슬라이스를 String으로 복원
        chars[start..start + max_len].iter().collect()
    }
}

/// left, right를 중심으로 좌우 대칭 비교하며 확장
fn expand(chars: &[char], mut left: usize, mut right: usize) -> (usize, usize) {
    let n = chars.len();

    // while 확장 가능하고 양쪽 문자가 같으면
    while left > 0 && right + 1 < n && chars[left - 1] == chars[right + 1] {
        left -= 1;
        right += 1;
    }

    (left, right)
}
