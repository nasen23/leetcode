impl Solution {
    pub fn is_match(s: String, p: String) -> bool {
        let (s, p) = (s.as_bytes(), p.as_bytes());
        let (mut ss, mut pp) = (0, 0);
        let mut star = None;
        let mut m = 0;
        while ss < s.len() {
            if pp < p.len() && (s[ss] == p[pp] || p[pp] == b'?') {
                ss += 1;
                pp += 1;
            } else if pp < p.len() && p[pp] == b'*' {
                star = Some(pp);
                m = ss;
                pp += 1;
            } else if let Some(star) = star {
                pp = star + 1;
                m += 1;
                ss = m;
            } else {
                return false;
            }
        }
        p[pp..].into_iter().all(|&x| x == b'*')
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn wildcard_match() {
        assert!(!Solution::is_match("aa".into(), "a".into()));
        assert!(Solution::is_match("aa".into(), "*".into()));
        assert!(!Solution::is_match("cb".into(), "?a".into()));
        assert!(Solution::is_match("adceb".into(), "*a*b".into()));
        assert!(!Solution::is_match("acdcb".into(), "a*c?b".into()));
    }
}
