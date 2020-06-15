impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        if strs.is_empty() {
            return "".into();
        }
        let mut res = String::new();
        let mut idx = 0;
        loop {
            let ch = match strs[0].as_bytes().get(idx) {
                Some(ch) => ch,
                None => break,
            };
            if strs
                .iter()
                .all(|s| s.as_bytes().get(idx).map_or(false, |c| c == ch))
            {
                res.push(*ch as char);
            } else {
                break;
            }
            idx += 1;
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
    fn longest_common_prefix() {
        assert_eq!(
            Solution::longest_common_prefix(vec_str!["flower", "flow", "flight"]),
            "fl"
        );
        assert_eq!(
            Solution::longest_common_prefix(vec_str!["dog", "racecar", "car"]),
            ""
        );
    }
}
