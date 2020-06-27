impl Solution {
    pub fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
        if matrix.is_empty() || matrix[0].is_empty() {
            return vec![];
        }
        let (r, c) = (matrix.len(), matrix[0].len());
        let (mut l, mut r, mut u, mut d) = (0, c - 1, 0, r - 1);
        let mut res = vec![];
        loop {
            for i in l..=r {
                res.push(matrix[u][i]);
            }
            u += 1;
            if u > d {
                break;
            }
            for i in u..=d {
                res.push(matrix[i][r]);
            }
            r = match r.checked_sub(1) {
                Some(r) => r,
                None => break,
            };
            if l > r {
                break;
            }
            for i in (l..=r).rev() {
                res.push(matrix[d][i]);
            }
            d = match d.checked_sub(1) {
                Some(d) => d,
                None => break,
            };
            if u > d {
                break;
            }
            for i in (u..=d).rev() {
                res.push(matrix[i][l]);
            }
            l += 1;
            if l > r {
                break;
            }
        }
        res
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vec_vec;

    #[test]
    fn spiral_matrix() {
        assert_eq!(
            Solution::spiral_order(vec_vec![[1, 2, 3], [4, 5, 6], [7, 8, 9]]),
            vec![1, 2, 3, 6, 9, 8, 7, 4, 5]
        );
        assert_eq!(Solution::spiral_order(vec_vec![[3], [2]]), vec![3, 2])
    }
}
