use crate::Solution;
use std::cmp::min;
use std::collections::{HashMap, VecDeque};

impl Solution {
    pub fn minimum_operations_to_make_equal_other(x: i32, y: i32) -> i32 {
        let mut dp = HashMap::new();

        Self::search(x, y, &mut dp)
    }

    pub fn search(mut x: i32, mut y: i32, dp: &mut HashMap<i32, i32>) -> i32 {
        if x <= y {
            y - x
        } else if dp.contains_key(&x) {
            dp[&x]
        } else {
            let mut r = x - y;

            let d = x % 5;

            r = min(r, Self::search((x - d) / 5, y, dp) + d + 1);

            let d = 5 - d;

            r = min(r, Self::search((x + d) / 5, y, dp) + d + 1);

            let d = x % 11;

            r = min(r, Self::search((x - d) / 11, y, dp) + d + 1);

            let d = 11 - d;

            r = min(r, Self::search((x + d) / 11, y, dp) + d + 1);

            dp.insert(x, r);

            r
        }
    }
    pub fn minimum_operations_to_make_equal(x: i32, y: i32) -> i32 {
        // x == y 이면 0번
        if x == y {
            return 0;
        }

        // 탐색 범위: 2 * max(x, y) + 여유
        let max_xy = x.max(y) as usize;
        let max_val = max_xy * 2 + 2;

        // dist[i] = 시작 x에서 i로 오기 위한 최소 연산 횟수, -1은 미방문
        let mut dist = vec![-1; max_val];
        let mut queue = VecDeque::new();

        let start = x as usize;
        let target = y as usize;
        dist[start] = 0;
        queue.push_back(start);

        while let Some(v) = queue.pop_front() {
            let d = dist[v];
            // 목표에 도달하면 바로 반환
            if v == target {
                return d;
            }

            // 1) decrement
            if v > 1 {
                let u = v - 1;
                if dist[u] == -1 {
                    dist[u] = d + 1;
                    queue.push_back(u);
                }
            }
            // 2) increment
            if v + 1 < max_val {
                let u = v + 1;
                if dist[u] == -1 {
                    dist[u] = d + 1;
                    queue.push_back(u);
                }
            }
            // 3) divide by 5
            if v % 5 == 0 {
                let u = v / 5;
                if dist[u] == -1 {
                    dist[u] = d + 1;
                    queue.push_back(u);
                }
            }
            // 4) divide by 11
            if v % 11 == 0 {
                let u = v / 11;
                if dist[u] == -1 {
                    dist[u] = d + 1;
                    queue.push_back(u);
                }
            }
        }

        // 혹시 목표에 도달하지 못하면 -1 반환 (문제 조건상 발생하지 않음)
        -1
    }

    pub fn minimum_operations_dp(x: i32, y: i32) -> i32 {
        // 탐색 범위를 2 * max(x, y) 정도로 잡아 충분히 여유를 둠
        let max_xy = x.max(y) as usize;
        let cap = max_xy * 2 + 2;
        // memo[i] == -2: 미계산, 그 외(>=0): 계산된 최소 연산 횟수
        let mut memo = vec![-2; cap];

        // 재귀 함수: i -> y 로 가는 최소 횟수
        fn dfs(i: usize, y: usize, memo: &mut [i32]) -> i32 {
            // 기저 사례 1: 동일하면 0
            if i == y {
                return 0;
            }
            // 기저 사례 2: i < y 이면 오직 +1 연산만 유효
            // (i에서 y까지 한 칸씩 올리기)
            if i < y {
                return (y - i) as i32;
            }
            // 이미 계산된 값이 있으면 바로 반환
            if memo[i] != -2 {
                return memo[i];
            }

            // 최악의 경우: i에서 y까지 모두 -1 연산
            let mut best = (i - y) as i32;

            // 나누기 연산 시도
            if i % 5 == 0 {
                best = min(best, 1 + dfs(i / 5, y, memo));
            }
            if i % 11 == 0 {
                best = min(best, 1 + dfs(i / 11, y, memo));
            }

            // +1, -1 연산도 시도 (예: 나누기 가능한 지점으로 가기 위해)
            best = min(best, 1 + dfs(i - 1, y, memo));
            if i + 1 < memo.len() {
                best = min(best, 1 + dfs(i + 1, y, memo));
            }

            memo[i] = best;
            best
        }

        dfs(x as usize, y as usize, &mut memo)
    }
}
