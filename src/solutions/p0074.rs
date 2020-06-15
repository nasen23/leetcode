impl Solution {
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        let r = matrix.len();
        if r == 0 {
            return false;
        }
        let c = matrix[0].len();
        if c == 0 {
            return false;
        }
        let (mut l, mut h) = (0, r * c - 1);
        macro_rules! unwrap_or_ret {
            ($op:expr, $val:expr) => {
                match $op {
                    Some(x) => x,
                    None => return $val,
                }
            };
        }
        loop {
            let mid = (l + h) / 2;
            let (r1, c1) = (mid / c, mid % c);
            if matrix[r1][c1] > target {
                h = unwrap_or_ret!(mid.checked_sub(1), false);
            } else if matrix[r1][c1] < target {
                l = mid + 1;
            } else {
                return true;
            }
            if l > h {
                return false;
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
    fn search_matrix() {
        assert!(Solution::search_matrix(
            vec_vec!([1, 3, 5, 7], [10, 11, 16, 20], [23, 30, 34, 50]),
            3
        ));
        assert!(!Solution::search_matrix(
            vec_vec!([1, 3, 5, 7], [10, 11, 16, 20], [23, 30, 34, 50]),
            13
        ));
        assert!(!Solution::search_matrix(vec_vec!([1]), 0));
        assert!(!Solution::search_matrix(vec_vec!([0]), 1));
    }
}
