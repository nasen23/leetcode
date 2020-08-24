impl Solution {
    pub fn repeated_substring_pattern(s: String) -> bool {
        let mut ss = s.clone();
        ss.push_str(&s);
        ss[1..ss.len() - 1].find(&s).is_some()
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn repeated_substring_pattern() {
        assert!(Solution::repeated_substring_pattern("abab".into()));
        assert!(!Solution::repeated_substring_pattern("a".into()));
        assert!(Solution::repeated_substring_pattern("abcabcabcabc".into()));
        assert!(Solution::repeated_substring_pattern("abcabc".into()));
    }
}
