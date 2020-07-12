impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut st = vec![];
        let map = |ch: char| match ch {
            ')' => '(',
            ']' => '[',
            '}' => '{',
            _ => unreachable!(),
        };
        for ch in s.chars() {
            match ch {
                ')' | ']' | '}' => match st.pop() {
                    Some(p) => {
                        if p != map(ch) {
                            return false;
                        }
                    }
                    None => return false,
                },
                _ => st.push(ch),
            }
        }
        st.is_empty()
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_parentheses() {
        assert_eq!(Solution::is_valid("()".into()), true);
        assert_eq!(Solution::is_valid("(]".into()), false);
        assert_eq!(Solution::is_valid("]".into()), false);
    }
}
