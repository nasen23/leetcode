impl Solution {
    pub fn min_path_sum(mut grid: Vec<Vec<i32>>) -> i32 {
        let (m, n) = (grid.len(), grid[0].len());
        for i in 0..m {
            for j in 0..n {
                grid[i][j] += match (i, j) {
                    (0, 0) => 0,
                    (0, _) => grid[i][j - 1],
                    (_, 0) => grid[i - 1][j],
                    _ => grid[i - 1][j].min(grid[i][j - 1]),
                }
            }
        }
        grid[m - 1][n - 1]
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vec_vec;

    #[test]
    fn minimum_path_sum() {
        assert_eq!(
            Solution::min_path_sum(vec_vec![[1, 3, 1], [1, 5, 1], [4, 2, 1]]),
            7
        );
    }
}
