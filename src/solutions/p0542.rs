use std::cmp;

impl Solution {
    pub fn update_matrix(matrix: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let (x, y) = (matrix.len(), matrix[0].len());
        let mut res = vec![vec![0; y]; x];

        for i in 0..x {
            for j in 0..y {
                if matrix[i][j] == 1 {
                    res[i][j] = 100000;
                    if i > 0 {
                        res[i][j] = cmp::min(res[i][j], 1 + res[i - 1][j]);
                    }
                    if j > 0 {
                        res[i][j] = cmp::min(res[i][j], 1 + res[i][j - 1]);
                    }
                }
            }
        }

        for i in (0..x).rev() {
            for j in (0..y).rev() {
                if matrix[i][j] == 1 {
                    if i < x - 1 {
                        res[i][j] = cmp::min(res[i][j], 1 + res[i + 1][j]);
                    }
                    if j < y - 1 {
                        res[i][j] = cmp::min(res[i][j], 1 + res[i][j + 1]);
                    }
                }
            }
        }

        res
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::vec_vec;

    #[test]
    fn matrix_01() {
        let input = vec_vec![[0, 0, 0], [0, 1, 0], [0, 0, 0]];
        let res = vec_vec![[0, 0, 0], [0, 1, 0], [0, 0, 0]];
        assert_eq!(Solution::update_matrix(input), res);
        let input = vec_vec![[0, 0, 0], [0, 1, 0], [1, 1, 1]];
        let res = vec_vec![[0, 0, 0], [0, 1, 0], [1, 2, 1]];
        assert_eq!(Solution::update_matrix(input), res);
        let input = vec_vec![
            [1, 0, 1, 1, 0, 0, 1, 0, 0, 1],
            [0, 1, 1, 0, 1, 0, 1, 0, 1, 1],
            [0, 0, 1, 0, 1, 0, 0, 1, 0, 0],
            [1, 0, 1, 0, 1, 1, 1, 1, 1, 1],
            [0, 1, 0, 1, 1, 0, 0, 0, 0, 1],
            [0, 0, 1, 0, 1, 1, 1, 0, 1, 0],
            [0, 1, 0, 1, 0, 1, 0, 0, 1, 1],
            [1, 0, 0, 0, 1, 1, 1, 1, 0, 1],
            [1, 1, 1, 1, 1, 1, 1, 0, 1, 0],
            [1, 1, 1, 1, 0, 1, 0, 0, 1, 1]
        ];
        let res = vec_vec![
            [1, 0, 1, 1, 0, 0, 1, 0, 0, 1],
            [0, 1, 1, 0, 1, 0, 1, 0, 1, 1],
            [0, 0, 1, 0, 1, 0, 0, 1, 0, 0],
            [1, 0, 1, 0, 1, 1, 1, 1, 1, 1],
            [0, 1, 0, 1, 1, 0, 0, 0, 0, 1],
            [0, 0, 1, 0, 1, 1, 1, 0, 1, 0],
            [0, 1, 0, 1, 0, 1, 0, 0, 1, 1],
            [1, 0, 0, 0, 1, 2, 1, 1, 0, 1],
            [2, 1, 1, 1, 1, 2, 1, 0, 1, 0],
            [3, 2, 2, 1, 0, 1, 0, 0, 1, 1]
        ];
        assert_eq!(Solution::update_matrix(input), res);
    }
}
