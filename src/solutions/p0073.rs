impl Solution {
    pub fn set_zeroes(matrix: &mut Vec<Vec<i32>>) {
        let (m, n) = (matrix.len(), matrix[0].len());
        let row = matrix[0].iter().any(|&x| x == 0);
        let col = matrix.iter().any(|v| v[0] == 0);
        for i in (1..m).rev() {
            for j in (1..n).rev() {
                if matrix[i][j] == 0 {
                    matrix[i][0] = 0;
                    matrix[0][j] = 0;
                }
            }
        }
        for i in (1..m).rev() {
            if matrix[i][0] == 0 {
                matrix[i].iter_mut().for_each(|x| *x = 0);
            }
        }
        for j in (1..n).rev() {
            if matrix[0][j] == 0 {
                matrix.iter_mut().for_each(|v| v[j] = 0);
            }
        }
        if row {
            matrix[0].iter_mut().for_each(|x| *x = 0);
        }
        if col {
            matrix.iter_mut().for_each(|v| v[0] = 0);
        }
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vec_vec;

    #[test]
    fn set_matrix_zeros() {
        let mut mat = vec_vec![[1, 1, 1], [1, 0, 1], [1, 1, 1]];
        Solution::set_zeroes(&mut mat);
        assert_eq!(mat, vec_vec![[1, 0, 1], [0, 0, 0], [1, 0, 1]]);
    }
}
