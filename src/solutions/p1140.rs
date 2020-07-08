impl Solution {
    pub fn stone_game_ii(piles: Vec<i32>) -> i32 {
        let mut dp = vec![vec![0; piles.len() + 1]; piles.len() + 1];
        let mut sum = 0;
        for i in (0..piles.len()).rev() {
            sum += piles[i];
            for m in 1..=piles.len() {
                if i + 2 * m >= piles.len() {
                    // if I could just take all the rest
                    dp[i][m] = sum;
                    continue;
                }
                for x in 1..=2 * m {
                    // dp[i + x][x.max(m)] is the max number of stones taken by enemy
                    // sum - dp[i + x][x.max(m)] is the number I can take
                    dp[i][m] = dp[i][m].max(sum - dp[i + x][x.max(m)]);
                }
            }
        }
        dp[0][1]
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stone_game_ii() {
        assert_eq!(Solution::stone_game_ii(vec![2, 7, 9, 4, 4]), 10);
    }
}
