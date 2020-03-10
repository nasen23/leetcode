struct Solution;

impl Solution {
    pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
        let mut dp = vec![std::i32::MAX; amount as usize + 1];
        dp[0] = 0;
        for i in 1..=amount {
            for c in &coins {
                if i - c >= 0 && dp[(i - c) as usize] != std::i32::MAX {
                    dp[i as usize] = std::cmp::min(dp[(i - c) as usize] + 1, dp[i as usize]);
                }
            }
        }

        if dp[amount as usize] == std::i32::MAX {
            -1
        } else {
            dp[amount as usize]
        }
    }
}
