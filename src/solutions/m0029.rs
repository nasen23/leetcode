impl Solution {
    pub fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
        if matrix.get(0).map_or(true, |v| v.is_empty()) {
            return vec![];
        }
        let (mut rl, mut rh) = (0isize, matrix.len() as isize - 1);
        let (mut cl, mut ch) = (0isize, matrix[0].len() as isize - 1);
        let mut res = Vec::with_capacity((rh as usize + 1) * (ch as usize + 1));

        loop {
            for i in cl..=ch {
                res.push(matrix[rl as usize][i as usize]);
            }
            rl += 1;
            if rl > rh {
                break;
            }
            for i in rl..=rh {
                res.push(matrix[i as usize][ch as usize]);
            }
            ch -= 1;
            if ch < cl {
                break;
            }
            for i in (cl..=ch).rev() {
                res.push(matrix[rh as usize][i as usize]);
            }
            rh -= 1;
            if rh < rl {
                break;
            }
            for i in (rl..=rh).rev() {
                res.push(matrix[i as usize][cl as usize]);
            }
            cl += 1;
            if cl > ch {
                break;
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
    fn spiral_order() {
        assert_eq!(
            Solution::spiral_order(vec_vec![[1, 2, 3], [4, 5, 6], [7, 8, 9]]),
            vec![1, 2, 3, 6, 9, 8, 7, 4, 5]
        );
        assert_eq!(Solution::spiral_order(vec_vec![[3], [2]]), vec![3, 2]);
    }
}
