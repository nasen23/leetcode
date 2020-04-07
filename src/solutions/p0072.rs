struct Solution;

use std::cmp;

impl Solution {
    pub fn min_distance(word1: String, word2: String) -> i32 {
        let word1 = word1.into_bytes();
        let word2 = word2.into_bytes();

        let mut cost = vec![vec![0; word2.len() + 1]; word1.len() + 1];
        for i in 0..=word1.len() {
            cost[i][0] = i as i32;
        }
        for j in 0..=word2.len() {
            cost[0][j] = j as i32;
        }

        for i in 1..=word1.len() {
            for j in 1..=word2.len() {
                if word1[i - 1] == word2[j - 1] {
                    cost[i][j] = cost[i - 1][j - 1];
                } else {
                    cost[i][j] =
                        1 + cmp::min(cost[i - 1][j - 1], cmp::min(cost[i - 1][j], cost[i][j - 1]));
                }
            }
        }

        cost[word1.len()][word2.len()]
    }
}

#[test]
fn test() {
    assert_eq!(Solution::min_distance("horse".into(), "ros".into()), 3);
}
