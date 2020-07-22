impl Solution {
    pub fn generate_matrix(n: i32) -> Vec<Vec<i32>> {
        let n = n as usize;
        let (mut l, mut r, mut u, mut d) = (0, n, 0, n);
        let mut res = vec![vec![0; n]; n];

        let mut k = 1;
        loop {
            for i in l..r {
                res[u][i] = k;
                k += 1;
            }
            u += 1;
            if u == d {
                break;
            }
            for i in u..d {
                res[i][r - 1] = k;
                k += 1;
            }
            r -= 1;
            if r == l {
                break;
            }
            for i in (l..r).rev() {
                res[d - 1][i] = k;
                k += 1;
            }
            d -= 1;
            if u == d {
                break;
            }
            for i in (u..d).rev() {
                res[i][l] = k;
                k += 1;
            }
            l += 1;
            if l == r {
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
    fn sprial_matrix2() {
        assert_eq!(
            Solution::generate_matrix(3),
            vec_vec![[1, 2, 3], [8, 9, 4], [7, 6, 5]]
        );
    }
}
