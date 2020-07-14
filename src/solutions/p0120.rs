impl Solution {
    pub fn minimum_total(triangle: Vec<Vec<i32>>) -> i32 {
        let mut dp = triangle.last().unwrap().clone();
        for r in triangle.into_iter().rev().skip(1) {
            for (i, x) in r.into_iter().enumerate() {
                dp[i] = x + dp[i].min(dp[i + 1]);
            }
        }
        dp[0]
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vec_vec;

    #[test]
    fn triangle() {
        assert_eq!(
            Solution::minimum_total(vec_vec!([2], [3, 4], [6, 5, 7], [4, 1, 8, 3])),
            11
        );
    }
}
