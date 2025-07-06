use crate::Solution;
use std::collections::HashSet;

impl Solution {
    pub fn can_finish(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> bool {
        let n = num_courses as usize;
        let mut graph = vec![vec![]; n];

        // 1. 그래프 구성: b -> a
        for pair in prerequisites {
            let a = pair[0] as usize;
            let b = pair[1] as usize;
            graph[b].push(a);
        }

        // 2. 방문 상태 배열: 0 = unvisited, 1 = visiting, 2 = visited
        let mut visited = vec![0; n];

        // 3. DFS로 사이클 판별
        fn has_cycle(u: usize, graph: &Vec<Vec<usize>>, visited: &mut Vec<i32>) -> bool {
            if visited[u] == 1 {
                return true; // 사이클 발생
            }
            if visited[u] == 2 {
                return false; // 이미 처리됨
            }

            visited[u] = 1; // 방문 중
            for &v in &graph[u] {
                if has_cycle(v, graph, visited) {
                    return true;
                }
            }
            visited[u] = 2; // 방문 완료
            false
        }

        // 4. 모든 노드에 대해 DFS 수행
        for i in 0..n {
            if has_cycle(i, &graph, &mut visited) {
                return false;
            }
        }
        true
    }
}
