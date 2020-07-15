impl Solution {
    pub fn num_trees(n: i32) -> i32 {
        let mut dp = vec![0; n as usize + 1];
        dp[0] = 1;
        dp[1] = 1;
        for i in 2..=n as usize {
            dp[i] = (0..i).fold(0, |acc, j| acc + dp[i - j - 1] * dp[j]);
        }
        dp[n as usize]
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unique_bsts() {
        assert_eq!(Solution::num_trees(3), 5);
    }
}
