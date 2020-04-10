struct Solution;

impl Solution {
    pub fn reverse_words(s: String) -> String {
        s.split_ascii_whitespace()
            .rev()
            .collect::<Vec<_>>()
            .join(" ")
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test() {
        assert_eq!(
            Solution::reverse_words("the sky is blue".into()),
            "blue is sky the"
        );
        assert_eq!(
            Solution::reverse_words("  hello world!  ".into()),
            "world! hello"
        );
        assert_eq!(
            Solution::reverse_words("a good   example".into()),
            "example good a"
        );
    }
}
