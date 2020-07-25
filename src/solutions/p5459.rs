impl Solution {
    pub fn min_number_operations(target: Vec<i32>) -> i32 {
        let mut dp = vec![0; target.len()];
        for i in 0..target.len() {
            if i > 0 {
                if target[i] > target[i - 1] {
                    dp[i] = dp[i - 1] + target[i] - target[i - 1];
                } else {
                    dp[i] = dp[i - 1];
                }
            } else {
                dp[i] = target[i];
            }
        }
        dp[target.len() - 1]
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn target_to_array_ops() {
        assert_eq!(Solution::min_number_operations(vec![1, 2, 3, 2, 1]), 3);
        assert_eq!(Solution::min_number_operations(vec![3, 1, 1, 2]), 4);
        assert_eq!(Solution::min_number_operations(vec![3, 1, 5, 4, 2]), 7);
        assert_eq!(Solution::min_number_operations(vec![1, 1, 1, 1]), 1);
    }
}
