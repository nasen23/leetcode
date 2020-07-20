impl Solution {
    pub fn my_sqrt(x: i32) -> i32 {
        if x <= 1 {
            return x;
        }
        let x = x as f64;
        let mut r = x;
        while r > x / r {
            r = ((r + x / r) as u64 / 2) as f64;
        }
        r as i32
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sqrtx() {
        assert_eq!(Solution::my_sqrt(4), 2);
        assert_eq!(Solution::my_sqrt(8), 2);
        assert_eq!(Solution::my_sqrt(7), 2);
        assert_eq!(Solution::my_sqrt(std::i32::MAX), 2);
    }
}
