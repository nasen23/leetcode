impl Solution {
    pub fn min_trio_degree(n: i32, edges: Vec<Vec<i32>>) -> i32 {
        let n = n as usize;
        let mut adj = vec![vec![false; n]; n];
        let mut deg = vec![0; n];
        for edge in edges {
            let (i, j) = (edge[0] as usize - 1, edge[1] as usize - 1);
            adj[i][j] = true;
            adj[j][i] = true;
            deg[i] += 1;
            deg[j] += 1;
        }
        let mut res = std::i32::MAX;
        for i in 0..n - 2 {
            for j in i + 1..n - 1 {
                if !adj[i][j] {
                    continue;
                }
                for k in j + 1..n {
                    if adj[i][k] && adj[j][k] {
                        res = res.min(deg[i] + deg[j] + deg[k] - 6);
                    }
                }
            }
        }
        if res == std::i32::MAX {
            -1
        } else {
            res
        }
    }
}

struct Solution;
