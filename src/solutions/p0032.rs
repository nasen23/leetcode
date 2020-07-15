impl Solution {
    pub fn longest_valid_parentheses(s: String) -> i32 {
        let s = s.as_bytes();
        let mut mark = vec![0; s.len()];
        let mut st = vec![];
        for (i, &b) in s.into_iter().enumerate() {
            match b {
                b'(' => st.push(i),
                _ => {
                    if let None = st.pop() {
                        mark[i] = 1;
                    }
                }
            }
        }
        for idx in st {
            mark[idx] = 1;
        }
        let (mut len, mut max) = (0, 0);
        for i in 0..mark.len() {
            if mark[i] == 1 {
                len = 0;
                continue;
            }
            len += 1;
            max = max.max(len);
        }
        max
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn longest_valid_parentheses() {
        assert_eq!(Solution::longest_valid_parentheses("(()".into()), 2);
        assert_eq!(Solution::longest_valid_parentheses(")()())".into()), 4);
        assert_eq!(Solution::longest_valid_parentheses(")(".into()), 0);
        assert_eq!(Solution::longest_valid_parentheses("()())".into()), 4);
        assert_eq!(Solution::longest_valid_parentheses("()())".into()), 4);
        assert_eq!(Solution::longest_valid_parentheses("()(()".into()), 2);
    }
}
