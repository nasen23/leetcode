use std::collections::VecDeque;

fn bfs(maze: &Vec<&[u8]>, x: usize, y: usize) -> Vec<Vec<i32>> {
    let mut res = vec![vec![-1; maze[0].len()]; maze.len()];
    res[x][y] = 0;
    let mut queue = VecDeque::new();
    queue.push_back((x, y));
    while let Some((i, j)) = queue.pop_front() {
        if i > 0 && maze[i - 1][j] != b'#' && res[i - 1][j] == -1 {
            res[i - 1][j] = res[i][j] + 1;
            queue.push_back((i - 1, j));
        }
        if j > 0 && maze[i][j - 1] != b'#' && res[i][j - 1] == -1 {
            res[i][j - 1] = res[i][j] + 1;
            queue.push_back((i, j - 1));
        }
        if i < maze.len() - 1 && maze[i + 1][j] != b'#' && res[i + 1][j] == -1 {
            res[i + 1][j] = res[i][j] + 1;
            queue.push_back((i + 1, j));
        }
        if j < maze[0].len() - 1 && maze[i][j + 1] != b'#' && res[i][j + 1] == -1 {
            res[i][j + 1] = res[i][j] + 1;
            queue.push_back((i, j + 1));
        }
    }
    res
}

impl Solution {
    pub fn minimal_steps(maze: Vec<String>) -> i32 {
        let maze = maze.iter().map(|s| s.as_bytes()).collect::<Vec<_>>();
        let (mut buttons, mut stones) = (vec![], vec![]);
        let (mut st, mut ed) = ((0, 0), (0, 0));
        for i in 0..maze.len() {
            for j in 0..maze[0].len() {
                match maze[i][j] {
                    b'S' => st = (i, j),
                    b'T' => ed = (i, j),
                    b'M' => buttons.push((i, j)),
                    b'O' => stones.push((i, j)),
                    _ => {}
                }
            }
        }
        let start_dist = bfs(&maze, st.0, st.1);
        if buttons.is_empty() {
            return start_dist[ed.0][ed.1];
        }
        let mut dist = vec![vec![-1; buttons.len() + 2]; buttons.len()];
        // dist[j][i] from i to a stone then to j min dist
        // dist[j][buttons.len()] from start pos to a stone then to j
        // dist[j][buttons.len() + 1] from j to end pos min dist
        let mut dd = vec![];
        for i in 0..buttons.len() {
            let d = bfs(&maze, buttons[i].0, buttons[i].1);
            dist[i][buttons.len() + 1] = d[ed.0][ed.1];
            dd.push(d);
        }
        for i in 0..buttons.len() {
            let mut tmp = -1;
            for k in 0..stones.len() {
                let (x, y) = stones[k];
                if dd[i][x][y] != -1 && start_dist[x][y] != -1 {
                    if tmp == -1 || tmp > dd[i][x][y] + start_dist[x][y] {
                        tmp = dd[i][x][y] + start_dist[x][y];
                    }
                }
            }
            dist[i][buttons.len()] = tmp;
            for j in i + 1..buttons.len() {
                let mut mn = -1;
                for k in 0..stones.len() {
                    let (x, y) = stones[k];
                    if dd[i][x][y] != -1 && dd[j][x][y] != -1 {
                        if mn == -1 || mn > dd[i][x][y] + dd[j][x][y] {
                            mn = dd[i][x][y] + dd[j][x][y];
                        }
                    }
                }
                dist[i][j] = mn;
                dist[j][i] = mn;
            }
        }
        for i in 0..buttons.len() {
            if dist[i][buttons.len()] == -1 || dist[i][buttons.len() + 1] == -1 {
                return -1;
            }
        }
        let mut dp = vec![vec![-1; buttons.len()]; 1 << buttons.len()];
        for i in 0..buttons.len() {
            dp[1 << i][i] = dist[i][buttons.len()];
        }
        for mask in 1..(1 << buttons.len()) {
            for i in 0..buttons.len() {
                if mask & (1 << i) != 0 {
                    for j in 0..buttons.len() {
                        if mask & (1 << j) == 0 {
                            let next = mask | (1 << j);
                            if dp[next][j] == -1 || dp[next][j] > dp[mask][i] + dist[i][j] {
                                dp[next][j] = dp[mask][i] + dist[i][j];
                            }
                        }
                    }
                }
            }
        }
        let mut ret = -1;
        let final_mask = (1 << buttons.len()) - 1;
        for i in 0..buttons.len() {
            // 状态是所有机关都已经打开了
            if ret == -1 || ret > dp[final_mask][i] + dist[i][buttons.len() + 1] {
                ret = dp[final_mask][i] + dist[i][buttons.len() + 1];
            }
        }
        ret
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vec_str;

    #[test]
    fn find_treasure() {
        assert_eq!(Solution::minimal_steps(vec_str!["S#O", "M..", "M.T"]), 16);
    }
}
