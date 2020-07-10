impl Solution {
    pub fn min_falling_path_sum(arr: Vec<Vec<i32>>) -> i32 {
        let mut dp = {
            let mut v = vec![vec![std::i32::MAX; arr.len()]; arr.len()];
            for i in 0..arr.len() {
                v[0][i] = arr[0][i];
            }
            v
        };
        for i in 1..arr.len() {
            for j in 0..arr.len() {
                for k in 0..arr.len() {
                    if j != k {
                        dp[i][j] = dp[i][j].min(arr[i][j] + dp[i - 1][k]);
                    }
                }
            }
        }
        *dp[arr.len() - 1].iter().min().unwrap()
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vec_vec;

    #[test]
    fn min_falling_path_sum_ii() {
        assert_eq!(
            Solution::min_falling_path_sum(vec_vec![[1, 2, 3], [4, 5, 6], [7, 8, 9]]),
            13
        );
    }
}
