impl Solution {
    pub fn monotone_increasing_digits(n: i32) -> i32 {
        let mut x = n;
        let mut i = 1;
        while i <= x / 10 {
            let d = x / i % 100;
            i *= 10;
            if d % 10 < d / 10 % 10 {
                x = x / i * i - 1;
            }
        }
        x
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn monotone_increasing_digits() {
        assert_eq!(Solution::monotone_increasing_digits(1332), 1299);
    }
}
