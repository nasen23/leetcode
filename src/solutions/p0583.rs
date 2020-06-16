impl Solution {
    pub fn min_distance(word1: String, word2: String) -> i32 {
        let (b1, b2) = (word1.as_bytes(), word2.as_bytes());
        let mut dp = vec![vec![0; b2.len() + 1]; b1.len() + 1];
        for i in 0..b1.len() {
            for j in 0..b2.len() {
                if b1[i] == b2[j] {
                    dp[i + 1][j + 1] = dp[i][j] + 1;
                } else {
                    dp[i + 1][j + 1] = std::cmp::max(dp[i][j + 1], dp[i + 1][j]);
                }
            }
        }

        (b1.len() + b2.len() - dp[b1.len()][b2.len()] * 2) as i32
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn min_distance() {
        assert_eq!(Solution::min_distance("sea".into(), "eat".into()), 2);
    }
}
