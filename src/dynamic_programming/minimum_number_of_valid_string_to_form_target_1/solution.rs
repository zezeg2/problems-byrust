use crate::Solution;
const ALPHABET_SIZE: usize = 26;

#[derive(Default)]
struct TrieNode {
    children: [Option<Box<TrieNode>>; ALPHABET_SIZE],
}

#[derive(Default)]
struct Trie {
    root: TrieNode,
}

impl Trie {
    fn insert(&mut self, word: &str) {
        let mut node = &mut self.root;
        for ch in word.as_bytes() {
            let idx = (ch - b'a') as usize;
            node = node.children[idx].get_or_insert_with(|| Box::new(TrieNode::default()));
        }
    }
}

impl Solution {
    pub fn min_valid_strings(words: Vec<String>, target: String) -> i32 {
        let n = target.len();
        let mut trie = Trie::default();

        // 1. Trie에 words의 단어를 정방향으로 삽입
        for word in &words {
            trie.insert(word);
        }

        // 2. DP 배열 초기화
        let mut dp = vec![i32::MAX; n + 1];
        dp[0] = 0;
        let target_bytes = target.as_bytes();

        // 3. target을 왼쪽에서 오른쪽으로 채움
        for i in 0..n {
            if dp[i] == i32::MAX {
                continue;
            }
            let mut current_node = &trie.root;
            for j in i..n {
                let char_index = (target_bytes[j] - b'a') as usize;
                if let Some(next_node) = current_node.children[char_index].as_ref() {
                    current_node = next_node;
                    dp[j + 1] = dp[j + 1].min(dp[i] + 1);
                } else {
                    break;
                }
            }
        }

        // 4. 결과 반환
        if dp[n] == i32::MAX { -1 } else { dp[n] }
    }


    pub fn min_valid_strings_timeout(words: Vec<String>, target: String) -> i32 {
        let n = target.len();
        // dp[i]는 target의 첫 i개 문자를 만드는 최소 비용
        let mut dp = vec![i32::MAX; n + 1];

        // 초기값: 빈 문자열을 만드는 비용은 0
        dp[0] = 0;

        // target의 모든 접두사 길이에 대해 반복 (1부터 n까지)
        for i in 1..=n {
            // 마지막 조각의 시작점을 찾기 위해 j를 0부터 i-1까지 반복
            for j in 0..i {
                // dp[j]가 유효한 경로일 경우에만 진행
                // (i32::MAX는 아직 도달할 수 없음을 의미)
                if dp[j] == i32::MAX {
                    continue;
                }

                // 마지막 조각이 될 부분 문자열
                let sub = &target[j..i];

                // 이 부분 문자열이 words에 있는 단어의 접두사인지 확인
                let is_valid_prefix = words.iter().any(|word| word.starts_with(sub));

                if is_valid_prefix {
                    // 유효하다면, dp[i]를 갱신
                    // dp[j] (이전까지의 최적 해) + 1 (현재 조각)
                    dp[i] = dp[i].min(dp[j] + 1);
                }
            }
        }

        // dp[n]이 초기값 그대로라면, target을 만들 수 없는 경우
        if dp[n] == i32::MAX {
            -1
        } else {
            dp[n]
        }
    }

    pub fn min_valid_strings_wrong(words: Vec<String>, target: String) -> i32 {
        let n = target.len();
        let mut dp = vec![0; n];
        let mut pivot = 0;
        let mut found = false;

        // 인덱스와 문자를 원래 순서대로 stack에 저장
        let mut st: Vec<(usize, char)> = target.chars().rev().enumerate().map(|(i, c)| (n - 1 - i, c)).collect();

        while let Some((i, _)) = st.pop() {
            let x = &target[pivot..=i];

            for word in &words {
                if word.starts_with(x) {
                    dp[i] = if x.len() == 1 {
                        if i == 0 { 1 } else { dp[i - 1] + 1 }
                    } else {
                        dp[i - 1]
                    };
                    found = true;
                    break;
                }
            }

            if !found {
                if x.len() == 1 {
                    return -1;
                }
                pivot = i;
                dp[i] = if i == 0 { 0 } else { dp[i - 1] };
                st.push((i, target.chars().nth(i).unwrap()));
            }

            found = false;
        }

        dp[n - 1]
    }
}

#[cfg(test)]

mod test {

    use super::*;
    #[test]
    fn case1() {
        let words = vec!["abc".to_string(), "aaaaa".to_string(), "bcdef".to_string()];
        let target = "aabcdabc".to_string();

        assert_eq!(Solution::min_valid_strings(words, target), 3);
    }
}
