impl Solution {
    pub fn is_bipartite(graph: Vec<Vec<i32>>) -> bool {
        let mut vis = vec![0; graph.len()];
        for i in 0..graph.len() {
            if vis[i] == 0 && !Solution::dfs(&graph, i, &mut vis, 1) {
                return false;
            }
        }
        true
    }

    fn dfs(graph: &Vec<Vec<i32>>, i: usize, vis: &mut Vec<i32>, c: i32) -> bool {
        vis[i] = c;
        for &x in &graph[i] {
            let x = x as usize;
            if vis[x] != 0 {
                if (c - vis[x]) % 2 == 0 {
                    return false;
                }
            } else if !Solution::dfs(graph, x, vis, c + 1) {
                return false;
            }
        }
        true
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vec_vec;

    #[test]
    fn is_graph_bipartite() {
        assert!(Solution::is_bipartite(vec_vec![
            [1, 3],
            [0, 2],
            [1, 3],
            [0, 2]
        ]));
    }
}
