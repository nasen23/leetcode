impl Solution {
    pub fn is_interleave(a: String, b: String, c: String) -> bool {
        if a.len() + b.len() != c.len() {
            return false;
        }
        let (a, b, c) = (a.as_bytes(), b.as_bytes(), c.as_bytes());
        let mut dp = vec![vec![false; b.len() + 1]; a.len() + 1];
        dp[0][0] = true;
        for i in 0..a.len() {
            dp[i + 1][0] = dp[i][0] && a[i] == c[i];
        }
        for j in 0..b.len() {
            dp[0][j + 1] = dp[0][j] && b[j] == c[j];
        }
        for i in 0..a.len() {
            for j in 0..b.len() {
                dp[i + 1][j + 1] = (dp[i][j + 1] && a[i] == c[i + j + 1])
                    || (dp[i + 1][j] && b[j] == c[i + j + 1]);
            }
        }
        dp[a.len()][b.len()]
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn interleaving_string() {
        assert!(Solution::is_interleave("".into(), "".into(), "".into()));
        assert!(Solution::is_interleave("a".into(), "b".into(), "ab".into()));
        assert!(Solution::is_interleave(
            "a".into(),
            "bb".into(),
            "bab".into()
        ));
        assert!(!Solution::is_interleave(
            "a".into(),
            "ba".into(),
            "aab".into()
        ));
        assert!(Solution::is_interleave(
            "aabcc".into(),
            "dbbca".into(),
            "aadbbcbcac".into()
        ));
        assert!(!Solution::is_interleave(
            "aabcc".into(),
            "dbbca".into(),
            "aadbbbaccc".into()
        ));
    }
}
