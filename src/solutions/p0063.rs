impl Solution {
    pub fn unique_paths_with_obstacles(obstacle_grid: Vec<Vec<i32>>) -> i32 {
        let (r, c) = (obstacle_grid.len(), obstacle_grid[0].len());
        let mut dp = vec![0; c];
        dp[0] = 1;
        for i in 0..r {
            for j in 0..c {
                if obstacle_grid[i][j] == 1 {
                    dp[j] = 0;
                } else {
                    dp[j] += j.checked_sub(1).map_or(0, |x| dp[x]);
                }
            }
        }
        dp[c - 1]
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vec_vec;

    #[test]
    fn unique_paths_ii() {
        assert_eq!(
            Solution::unique_paths_with_obstacles(vec_vec![[0, 0, 0], [0, 1, 0], [0, 0, 0]]),
            2
        )
    }
}
