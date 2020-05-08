use std::cmp;

impl Solution {
    pub fn maximal_square(matrix: Vec<Vec<char>>) -> i32 {
        if matrix.len() == 0 {
            return 0;
        }

        let (r, c) = (matrix.len(), matrix[0].len());
        let mut dp = vec![vec![0; c]; r];
        let mut max = 0;
        for i in 0..r {
            for j in 0..c {
                if matrix[i][j] == '1' {
                    let (a, b, c) = (
                        *dp.get(i - 1).and_then(|v| v.get(j)).unwrap_or(&0),
                        *dp.get(i - 1).and_then(|v| v.get(j - 1)).unwrap_or(&0),
                        *dp.get(i).and_then(|v| v.get(j - 1)).unwrap_or(&0),
                    );

                    dp[i][j] = cmp::min(a, cmp::min(b, c)) + 1;
                    max = cmp::max(max, dp[i][j]);
                }
            }
        }

        max * max
    }
}

struct Solution;
