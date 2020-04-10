struct Solution;

impl Solution {
    pub fn unique_paths(m: i32, n: i32) -> i32 {
        let mut ve = vec![vec![0; n as usize]; m as usize];

        for i in 0..m as usize {
            ve[i][0] = 1;
        }
        for j in 0..n as usize {
            ve[0][j] = 1;
        }

        for i in 1..m as usize {
            for j in 1..n as usize {
                ve[i][j] = ve[i - 1][j] + ve[i][j - 1];
            }
        }

        ve[m as usize - 1][n as usize - 1]
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test() {
        assert_eq!(Solution::unique_paths(3, 2), 3);
    }
}
