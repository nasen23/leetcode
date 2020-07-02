impl Solution {
    pub fn kth_smallest(matrix: Vec<Vec<i32>>, k: i32) -> i32 {
        let n = matrix.len();
        let (mut left, mut right) = (matrix[0][0], matrix[n - 1][n - 1]);
        while left < right {
            let mid = left + (right - left) / 2;
            if check(&matrix, mid, k) {
                right = mid;
            } else {
                left = mid + 1;
            }
        }
        left
    }
}

fn check(mat: &Vec<Vec<i32>>, mid: i32, k: i32) -> bool {
    let n = mat.len();
    let (mut i, mut j) = (n - 1, 0);
    let mut num = 0;
    while j < n {
        if mat[i][j] <= mid {
            num += i + 1;
            j += 1;
        } else {
            i = match i.checked_sub(1) {
                Some(i) => i,
                None => break,
            }
        }
    }
    num >= k as usize
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vec_vec;

    #[test]
    fn kth_smallest() {
        assert_eq!(
            Solution::kth_smallest(vec_vec![[1, 5, 9], [10, 11, 13], [12, 13, 15]], 8),
            13
        )
    }
}
