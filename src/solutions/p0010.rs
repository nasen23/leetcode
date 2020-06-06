impl Solution {
    pub fn is_match<S: AsRef<str>>(s: S, p: S) -> bool {
        is_match_impl(s.as_ref().as_bytes(), p.as_ref().as_bytes())
    }
}

fn is_match_impl(src: &[u8], re: &[u8]) -> bool {
    if re.is_empty() {
        return src.is_empty();
    }
    let is_match = !src.is_empty() && (src[0] == re[0] || re[0] == b'.');
    if re.get(1) == Some(&b'*') {
        is_match_impl(src, &re[2..]) || (is_match && is_match_impl(&src[1..], re))
    } else {
        is_match && is_match_impl(&src[1..], &re[1..])
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn is_match() {
        assert!(Solution::is_match("abcd", "abcd"));
        assert!(!Solution::is_match("aa", "a"));
        assert!(Solution::is_match("aa", "a*"));
        assert!(Solution::is_match("ab", ".*"));
        assert!(Solution::is_match("aab", "c*a*b"));
        assert!(!Solution::is_match("mississippi", "mis*is*p*."));
    }
}
