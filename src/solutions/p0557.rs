impl Solution {
    pub fn reverse_words(s: String) -> String {
        s.split_ascii_whitespace()
            .map(|s| s.chars().rev().collect::<String>())
            .collect::<Vec<_>>()
            .join(" ")
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reverse_words_in_string3() {
        assert_eq!(
            Solution::reverse_words("Let's take LeetCode contest".into()),
            "s'teL ekat edoCteeL tsetnoc"
        );
    }
}
