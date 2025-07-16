use std::collections::HashMap;
use crate::Solution;

impl Solution {
    pub fn sum_of_distances_in_tree(n: i32, edges: Vec<Vec<i32>>) -> Vec<i32> {
        let n = n as usize;
        let mut graph = vec![vec![]; n];

        for edge in edges {
            let (u, v) = (edge[0] as usize, edge[1] as usize);
            graph[u].push(v);
            graph[v].push(u);
        }

        let mut res = vec![0; n];     // 결과 (거리 합)
        let mut count = vec![1; n];   // 서브트리 노드 수

        // 1. Post-order DFS
        fn post_order(
            node: usize,
            parent: Option<usize>,
            graph: &Vec<Vec<usize>>,
            res: &mut Vec<i32>,
            count: &mut Vec<i32>,
        ) {
            for &child in &graph[node] {
                if Some(child) == parent {
                    continue;
                }

                post_order(child, Some(node), graph, res, count);

                count[node] += count[child];
                res[node] += res[child] + count[child];
            }
        }

        // 2. Pre-order DFS
        fn pre_order(
            node: usize,
            parent: Option<usize>,
            graph: &Vec<Vec<usize>>,
            res: &mut Vec<i32>,
            count: &Vec<i32>,
        ) {
            for &child in &graph[node] {
                if Some(child) == parent {
                    continue;
                }

                res[child] = res[node] - count[child] + (count.len() as i32 - count[child]);
                pre_order(child, Some(node), graph, res, count);
            }
        }

        post_order(0, None, &graph, &mut res, &mut count);
        pre_order(0, None, &graph, &mut res, &count);

        res
    }
}


#[cfg(test)]
mod test {
    #[test]
    fn case1() {
        use super::*;
        let n = 6;
        let edges = vec![vec![0, 1], vec![0, 2], vec![2, 3], vec![2, 4], vec![2, 5]];

        assert_eq!(Solution::sum_of_distances_in_tree(n,edges), vec![8,12,6,10,10,10])
    }
}