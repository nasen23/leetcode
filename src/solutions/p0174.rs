impl Solution {
    pub fn calculate_minimum_hp(dungeon: Vec<Vec<i32>>) -> i32 {
        // reverse dp
        let mut dp = vec![vec![0; dungeon[0].len()]; dungeon.len()];
        for i in (0..dungeon.len()).rev() {
            for j in (0..dungeon[0].len()).rev() {
                dp[i][j] = 1.max(
                    match (dp.get(i + 1).map(|r| r[j]), dp[i].get(j + 1).cloned()) {
                        (Some(x), Some(y)) => x.min(y),
                        (Some(x), None) => x,
                        (None, Some(y)) => y,
                        (None, None) => 0,
                    } - dungeon[i][j]
                        + if i == dungeon.len() - 1 && j == dungeon[0].len() - 1 {
                            1
                        } else {
                            0
                        },
                );
            }
        }
        dp[0][0]
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vec_vec;

    #[test]
    fn dungeon_game() {
        assert_eq!(
            Solution::calculate_minimum_hp(vec_vec![[-2, -3, 3], [-5, -10, 1], [10, 30, -5]]),
            7
        );
    }
}
