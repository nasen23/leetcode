struct Solution;

impl Solution {
    pub fn convert_to_title(n: i32) -> String {
        let mut s = String::with_capacity(8);
        let mut n = n;
        while n != 0 {
            let c = (((n - 1) % 26) as u8 + b'A') as char;
            s.insert(0, c);
            n = (n - 1) / 26;
        }

        s
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test() {
        assert_eq!(Solution::convert_to_title(1), "A");
        assert_eq!(Solution::convert_to_title(26), "Z");
        assert_eq!(Solution::convert_to_title(28), "AB");
        assert_eq!(Solution::convert_to_title(701), "ZY");
    }
}
