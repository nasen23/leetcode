impl Solution {
    pub fn find_length(a: Vec<i32>, b: Vec<i32>) -> i32 {
        let mut dp = vec![0; b.len() + 1];
        let mut max = 0;
        for i in 0..a.len() {
            for j in (0..b.len()).rev() {
                if a[i] == b[j] {
                    dp[j + 1] = dp[j] + 1;
                    max = max.max(dp[j + 1]);
                }
            }
        }
        max
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_length() {
        assert_eq!(
            Solution::find_length(vec![1, 2, 3, 2, 1], vec![3, 2, 1, 4, 1]),
            3
        );
        assert_eq!(
            Solution::find_length(vec![1, 2, 3, 4, 5], vec![1, 2, 3, 4, 5]),
            5
        );
        assert_eq!(
            Solution::find_length(vec![5, 4, 3, 2, 1], vec![1, 2, 3, 4, 5]),
            1
        );
        assert_eq!(
            Solution::find_length(vec![0, 1, 1, 1, 1], vec![1, 0, 1, 0, 1]),
            2
        );
    }
}
