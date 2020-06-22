impl Solution {
    pub fn reverse(x: i32) -> i32 {
        let mut res: i32 = 0;
        let mut x = x;
        while x != 0 {
            res = match res.checked_mul(10).and_then(|n| n.checked_add(x % 10)) {
                Some(res) => res,
                None => return 0,
            };
            x /= 10;
        }
        res
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reverse_integer() {
        assert_eq!(Solution::reverse(10), 1);
        assert_eq!(Solution::reverse(-123), -321);
    }
}
