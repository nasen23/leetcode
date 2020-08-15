impl Solution {
    pub fn remove_boxes(boxes: Vec<i32>) -> i32 {
        let mut dp = vec![vec![vec![0; 105]; 105]; 105];
        for len in 1..=boxes.len() {
            for i in 0..boxes.len() - len + 1 {
                let j = i + len - 1;
                for k in 0..boxes.len() {
                    dp[i][j][k] = dp[i][j][k]
                        .max(if j == i { 0 } else { dp[i][j - 1][0] } + ((k + 1) * (k + 1)) as i32);
                    for t in i..j {
                        if boxes[t] == boxes[j] {
                            dp[i][j][k] = dp[i][j][k].max(
                                if t + 2 > j { 0 } else { dp[t + 1][j - 1][0] } + dp[i][t][k + 1],
                            )
                        }
                    }
                }
            }
        }
        dp[0][boxes.len() - 1][0]
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn remove_boxes() {
        assert_eq!(Solution::remove_boxes(vec![1, 3, 2, 2, 2, 3, 4, 3, 1]), 23);
        assert_eq!(Solution::remove_boxes(vec![6, 3, 6, 5, 6, 6, 6, 6]), 38);
        assert_eq!(
            Solution::remove_boxes(vec![
                86, 26, 80, 27, 1, 16, 78, 71, 36, 52, 65, 76, 58, 77, 45, 17, 100, 37, 37, 75, 49,
                2, 37, 42, 19, 99, 14, 33, 34, 58, 4, 30, 100, 88, 74, 47, 80, 77, 85, 32, 80, 35,
                80, 25, 60, 91, 99, 27, 47, 66, 13, 20, 15, 10, 26, 39, 60, 9, 63, 24, 66, 32, 29,
                79, 67, 19, 88, 35, 44, 67, 22, 99, 27, 27, 40, 78, 2, 21, 40, 69, 88, 26, 57, 23,
                15, 70, 1, 100, 37, 20, 26, 18, 27, 86, 88, 33, 28, 40, 92, 15
            ]),
            38
        );
    }
}
