impl Solution {
    pub fn num_decodings(s: String) -> i32 {
        let s = s.as_bytes();
        let (mut pp, mut p) = (1, (s[0] != b'0') as i32);
        for i in 1..s.len() {
            let mut c = 0;
            if s[i] != b'0' {
                c += p;
            }
            if s[i - 1] == b'1' || (s[i - 1] == b'2' && s[i] <= b'6') {
                c += pp;
            }
            pp = p;
            p = c;
        }
        p
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decode_ways() {
        assert_eq!(Solution::num_decodings("12".into()), 2);
        assert_eq!(Solution::num_decodings("0".into()), 0);
        assert_eq!(Solution::num_decodings("226".into()), 3);
    }
}
