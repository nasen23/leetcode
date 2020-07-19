impl Solution {
    pub fn max_coins(nums: Vec<i32>) -> i32 {
        // REVIEW: range dp
        let mut val = vec![1; nums.len() + 2];
        val[0] = 1;
        val[nums.len() + 1] = 1;
        for i in 0..nums.len() {
            val[i + 1] = nums[i];
        }
        let mut dp = vec![vec![0; nums.len() + 2]; nums.len() + 2];
        for i in (0..nums.len()).rev() {
            for j in i + 2..nums.len() + 2 {
                for k in i + 1..j {
                    dp[i][j] = dp[i][j].max(dp[i][k] + dp[k][j] + val[i] * val[j] * val[k]);
                }
            }
        }
        dp[0][nums.len() + 1]
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn burst_balloons() {
        assert_eq!(Solution::max_coins(vec![3, 1, 5, 8]), 167);
    }
}
