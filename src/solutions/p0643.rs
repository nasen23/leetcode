impl Solution {
    pub fn find_max_average(nums: Vec<i32>, k: i32) -> f64 {
        let mut sums = vec![0; nums.len() + 1];
        for (i, x) in nums.into_iter().enumerate() {
            sums[i + 1] = sums[i] + x;
        }
        (0..sums.len() - k as usize)
            .map(|i| (sums[i + k as usize] - sums[i]) as f64 / k as f64)
            .max_by(|a, b| a.partial_cmp(b).unwrap())
            .unwrap_or(0.0)
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn maximum_average_subarray_i() {
        assert_eq!(
            Solution::find_max_average(vec![1, 12, -5, -6, 50, 3], 4),
            12.75
        );
        assert_eq!(Solution::find_max_average(vec![1], 1), 1.0);
    }
}
