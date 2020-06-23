impl Solution {
    pub fn integer_replacement(n: i32) -> i32 {
        if n == std::i32::MAX {
            return 32;
        }
        let mut n = n;
        let mut res = 0;
        while n > 1 {
            if n % 2 == 0 {
                n /= 2;
            } else if n & 2 == 2 && n > 3 {
                n += 1;
            } else {
                n -= 1;
            }
            res += 1;
        }
        res
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn integer_replacement() {
        assert_eq!(Solution::integer_replacement(7), 4);
    }
}
