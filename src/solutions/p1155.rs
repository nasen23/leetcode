impl Solution {
    pub fn num_rolls_to_target(d: i32, f: i32, target: i32) -> i32 {
        if target < d || d * f < target {
            return 0;
        }
        if d == 1 || target == d || d * f == target {
            return 1;
        }
        let target = target as usize;
        let m = 10i32.pow(9) + 7;
        let mut dp = vec![0; target + 1];
        dp[0] = 1;
        for i in 1..=d {
            for j in (0..=std::cmp::min(target, (i * f) as usize)).rev() {
                dp[j] = 0;
                for k in 1..=std::cmp::min(f as usize, j) {
                    dp[j] = (dp[j] + dp[j - k]) % m;
                }
            }
        }

        dp[target]
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn num_rolls_to_target() {
        assert_eq!(Solution::num_rolls_to_target(1, 6, 3), 1);
        assert_eq!(Solution::num_rolls_to_target(2, 6, 7), 6);
        assert_eq!(Solution::num_rolls_to_target(2, 5, 10), 1);
        assert_eq!(Solution::num_rolls_to_target(1, 2, 3), 0);
        assert_eq!(Solution::num_rolls_to_target(30, 30, 500), 222616187);
    }
}
