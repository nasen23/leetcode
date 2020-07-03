impl Solution {
    pub fn max_score(s: String) -> i32 {
        let mut sum = s.chars().filter(|&ch| ch == '1').count() as i32;
        s.chars()
            .take(s.len() - 1)
            .map(|ch| {
                sum += if ch == '1' { -1 } else { 1 };
                sum
            })
            .max()
            .unwrap()
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn max_score_after_split_string() {
        assert_eq!(Solution::max_score("011101".into()), 5)
    }
}
