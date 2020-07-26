impl Solution {
    pub fn longest_increasing_path(matrix: Vec<Vec<i32>>) -> i32 {
        if matrix.is_empty() || matrix[0].is_empty() {
            return 0;
        }
        let (m, n) = (matrix.len(), matrix[0].len());
        let mut lst = Vec::with_capacity(m * n);
        for i in 0..matrix.len() {
            for j in 0..matrix[0].len() {
                lst.push((matrix[i][j], i, j));
            }
        }
        lst.sort_unstable_by_key(|t| t.0);
        let mut path = vec![vec![1; n]; m];
        for (_, i, j) in lst.into_iter().rev() {
            if i > 0 && matrix[i][j] < matrix[i - 1][j] {
                path[i][j] = path[i][j].max(path[i - 1][j] + 1);
            }
            if j > 0 && matrix[i][j] < matrix[i][j - 1] {
                path[i][j] = path[i][j].max(path[i][j - 1] + 1);
            }
            if i < m - 1 && matrix[i][j] < matrix[i + 1][j] {
                path[i][j] = path[i][j].max(path[i + 1][j] + 1);
            }
            if j < n - 1 && matrix[i][j] < matrix[i][j + 1] {
                path[i][j] = path[i][j].max(path[i][j + 1] + 1);
            }
        }
        path.into_iter().flatten().max().unwrap()
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vec_vec;

    #[test]
    fn longest_increasing_path_in_mat() {
        assert_eq!(
            Solution::longest_increasing_path(vec_vec![[9, 9, 4], [6, 6, 8], [2, 1, 1]]),
            4
        );
        assert_eq!(
            Solution::longest_increasing_path(vec_vec![[3, 4, 5], [3, 2, 6], [2, 2, 1]]),
            4
        );
        assert_eq!(Solution::longest_increasing_path(vec_vec![[1, 2]]), 2);
    }
}
