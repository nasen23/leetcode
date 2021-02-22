impl Solution {
    pub fn is_toeplitz_matrix(matrix: Vec<Vec<i32>>) -> bool {
        let n = matrix[0].len();
        matrix
            .iter()
            .zip(matrix.iter().skip(1))
            .all(|(a, b)| a.iter().take(n - 1).eq(b.iter().skip(1)))
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vec_vec;

    #[test]
    fn toeplitz_matrix() {
        assert!(Solution::is_toeplitz_matrix(vec_vec![
            [1, 2, 3, 4],
            [5, 1, 2, 3],
            [9, 5, 1, 2]
        ]));
        assert!(!Solution::is_toeplitz_matrix(vec_vec![[1, 2], [2, 2]]));
        assert!(!Solution::is_toeplitz_matrix(vec_vec![
            [36, 59, 71, 15, 26, 82, 87],
            [56, 36, 59, 71, 15, 26, 82],
            [15, 0, 36, 59, 71, 15, 26]
        ]));
    }
}
