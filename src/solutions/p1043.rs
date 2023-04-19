impl Solution {
    pub fn max_sum_after_partitioning(arr: Vec<i32>, k: i32) -> i32 {
        let mut dp = vec![0; arr.len() + 1];
        for i in 1..=arr.len() {
            for j in 1..=(k as usize).min(i) as usize {
                dp[i] =
                    dp[i].max(dp[i - j] + arr[i - j..i].iter().max().copied().unwrap() * j as i32);
            }
        }
        dp[arr.len()]
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn partition_array_for_maximum_sum() {
        assert_eq!(
            Solution::max_sum_after_partitioning(vec![1, 15, 7, 9, 2, 5, 10], 3),
            84
        );
        assert_eq!(
            Solution::max_sum_after_partitioning(vec![1, 4, 1, 5, 7, 3, 6, 1, 9, 9, 3], 4),
            83
        );
    }
}
