impl Solution {
    pub fn count_binary_substrings(s: String) -> i32 {
        let (mut last, mut cur) = (0, 1);
        let mut res = 0;
        let s = s.as_bytes();
        for i in 1..s.len() {
            if s[i] == s[i - 1] {
                cur += 1;
            } else {
                last = cur;
                cur = 1;
            }
            if last >= cur {
                res += 1;
            }
        }
        res
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn count_binary_strings() {
        assert_eq!(Solution::count_binary_substrings("00110011".into()), 6);
        assert_eq!(Solution::count_binary_substrings("10101".into()), 4);
    }
}
