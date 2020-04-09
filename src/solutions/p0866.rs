struct Solution;

impl Solution {
    pub fn prime_palindrome(n: i32) -> i32 {
        let is_prime = |x: i32| {
            if x <= 3 {
                return x > 1;
            }
            if x % 2 == 0 || x % 3 == 0 {
                return false;
            }

            let k = (x as f64).sqrt() as i32 + 1;
            for i in (5..k).step_by(6) {
                if x % i == 0 || x % (i + 2) == 0 {
                    return false;
                }
            }

            true
        };

        let is_palindrome = |x: i32| {
            let s = x.to_string();
            s == s.chars().rev().collect::<String>()
        };

        match n {
            _ if n <= 2 => 2,
            3 => 3,
            _ if n <= 5 => 5,
            _ if n <= 7 => 7,
            _ if n <= 11 => 11,
            mut i => loop {
                if is_palindrome(i) && is_prime(i) {
                    break i;
                }
                i += 1;
                let len = i.to_string().as_bytes().len();
                if len % 2 == 0 {
                    i = 10i32.pow(len as u32);
                }
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test() {
        assert_eq!(Solution::prime_palindrome(6), 7);
        assert_eq!(Solution::prime_palindrome(8), 11);
        assert_eq!(Solution::prime_palindrome(13), 101);
        assert_eq!(Solution::prime_palindrome(102), 131);
        assert_eq!(Solution::prime_palindrome(9989900), 100030001);
    }
}
