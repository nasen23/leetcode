impl Solution {
    pub fn full_justify(words: Vec<String>, max_width: i32) -> Vec<String> {
        let mut res = vec![];
        let mut words = &words[..];
        loop {
            if words.is_empty() {
                break;
            }
            let mut line = String::new();
            let mut i = 0;
            let mut len = 0;
            while i < words.len() && len + words[i].len() + i <= max_width as usize {
                len += words[i].len();
                i += 1;
            }
            let whitespace = max_width as usize - len;
            let (each, one_more) = if i == words.len() {
                (1, 0)
            } else if i > 1 {
                (whitespace / (i - 1), whitespace % (i - 1))
            } else {
                (whitespace, 0)
            };
            for j in 0..i {
                line.push_str(&words[j]);
                if j < i - 1 {
                    for _ in 0..each {
                        line.push(' ');
                    }
                    if j < one_more {
                        line.push(' ');
                    }
                }
            }
            if line.len() < max_width as usize {
                for _ in 0..max_width as usize - line.len() {
                    line.push(' ');
                }
            }
            words = &words[i..];
            res.push(line);
        }
        res
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vec_str;

    #[test]
    fn text_justification() {
        assert_eq!(
            Solution::full_justify(
                vec_str![
                    "This",
                    "is",
                    "an",
                    "example",
                    "of",
                    "text",
                    "justification."
                ],
                16
            ),
            vec_str!["This    is    an", "example  of text", "justification.  "]
        );
        assert_eq!(
            Solution::full_justify(
                vec_str!["What", "must", "be", "acknowledgment", "shall", "be"],
                16
            ),
            vec_str!["What   must   be", "acknowledgment  ", "shall be        "]
        );
        assert_eq!(
            Solution::full_justify(
                vec_str![
                    "Science",
                    "is",
                    "what",
                    "we",
                    "understand",
                    "well",
                    "enough",
                    "to",
                    "explain",
                    "to",
                    "a",
                    "computer.",
                    "Art",
                    "is",
                    "everything",
                    "else",
                    "we",
                    "do"
                ],
                20
            ),
            vec_str![
                "Science  is  what we",
                "understand      well",
                "enough to explain to",
                "a  computer.  Art is",
                "everything  else  we",
                "do                  "
            ]
        );
    }
}
