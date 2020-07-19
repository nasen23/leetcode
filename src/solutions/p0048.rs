impl Solution {
    pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
        matrix.reverse();
        for i in 0..matrix.len() {
            for j in i + 1..matrix.len() {
                let t = matrix[i][j];
                matrix[i][j] = matrix[j][i];
                matrix[j][i] = t;
            }
        }
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vec_vec;

    #[test]
    fn rotate_image() {
        let mut v = vec_vec![[1, 2, 3], [4, 5, 6], [7, 8, 9]];
        Solution::rotate(&mut v);
        assert_eq!(v, vec_vec![[7, 4, 1], [8, 5, 2], [9, 6, 3]]);
    }
}
