struct Solution;

impl Solution {
    pub fn nth_super_ugly_number(n: i32, primes: Vec<i32>) -> i32 {
        let mut dp = vec![std::i32::MAX; n as usize + 1];
        let mut flag = vec![0; primes.len()];
        dp[0] = 1;

        for i in 1..n {
            for j in 0..primes.len() {
                if dp[i as usize] > primes[j] * dp[flag[j]] {
                    dp[i as usize] = primes[j] * dp[flag[j]];
                }
            }
            for j in 0..primes.len() {
                if dp[i as usize] >= primes[j] * dp[flag[j]] {
                    flag[j] += 1;
                }
            }
        }

        dp[n as usize - 1]
    }
}
