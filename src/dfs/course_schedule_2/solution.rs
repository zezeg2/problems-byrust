use crate::Solution;

impl Solution {
    pub fn find_order(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> Vec<i32> {
        let mut solver = Solver::new(num_courses as usize, prerequisites);
        solver.solve()
    }
}

// 방문 상태를 의미로 표현하는 enum
#[derive(Clone, Copy, PartialEq, Eq)]
enum VisitState {
    Unvisited,
    Visiting,
    Visited,
}

struct Solver {
    graph: Vec<Vec<usize>>,
    visited: Vec<VisitState>,
    order: Vec<usize>,
}

impl Solver {
    fn new(num_courses: usize, prerequisites: Vec<Vec<i32>>) -> Self {
        let mut graph = vec![vec![]; num_courses];

        for pair in prerequisites {
            let (to, from) = (pair[0] as usize, pair[1] as usize);
            graph[from].push(to);
        }

        Solver {
            graph,
            visited: vec![VisitState::Unvisited; num_courses],
            order: Vec::with_capacity(num_courses),
        }
    }

    fn solve(&mut self) -> Vec<i32> {
        for u in 0..self.graph.len() {
            if self.visited[u] == VisitState::Unvisited {
                if !self.dfs(u) {
                    return vec![]; // 사이클 발생 시 빈 벡터 반환
                }
            }
        }

        self.order.reverse();
        self.order.iter().map(|&x| x as i32).collect()
    }

    fn dfs(&mut self, u: usize) -> bool {
        match self.visited[u] {
            VisitState::Visiting => return false, // 사이클 발생
            VisitState::Visited => return true,   // 이미 처리된 노드
            VisitState::Unvisited => {}           // 계속 진행
        }

        self.visited[u] = VisitState::Visiting;

        let neighbors: Vec<_> = self.graph[u].clone();
        for &v in &neighbors {
            if !self.dfs(v) {
                return false;
            }
        }

        self.visited[u] = VisitState::Visited;
        self.order.push(u);
        true
    }
}