use std::collections::HashSet;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let (mut i, mut j) = (0, 0);
        let bytes = s.as_bytes();
        let mut set = HashSet::new();

        let mut len = 0;
        while j < s.len() {
            while j < s.len() && !set.contains(&bytes[j]) {
                set.insert(bytes[j]);
                j += 1;
            }

            len = std::cmp::max(len, j - i);
            set.remove(&bytes[i]);
            i += 1;
        }

        len as i32
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn length_of_longest_substring() {
        assert_eq!(Solution::length_of_longest_substring("abcabcbb".into()), 3);
        assert_eq!(Solution::length_of_longest_substring("bbbbb".into()), 1);
        assert_eq!(Solution::length_of_longest_substring("pwwkew".into()), 3);
        assert_eq!(Solution::length_of_longest_substring("dvdf".into()), 3);
    }
}
