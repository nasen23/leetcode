impl Solution {
    pub fn my_pow(x: f64, n: i32) -> f64 {
        match n {
            0 => 1.0,
            n if n > 0 && n % 2 == 0 => Solution::my_pow(x * x, n / 2),
            n if n > 0 => Solution::my_pow(x * x, n / 2) * x,
            n if n == std::i32::MIN => 1.0 / (Solution::my_pow(x, std::i32::MAX) * x),
            _ => 1.0 / Solution::my_pow(x, -n),
        }
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn my_pow() {
        assert_eq!(Solution::my_pow(2.00, 10), 1024.0);
    }
}
