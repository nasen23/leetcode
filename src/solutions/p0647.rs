impl Solution {
    pub fn count_substrings(s: String) -> i32 {
        if s.is_empty() {
            return 0;
        }
        let s = s.as_bytes();
        let mut res = 0;
        for pos in 0..2 * s.len() - 1 {
            let (mut l, mut r) = ((pos + 2) / 2, (pos + 1) / 2);
            while l > 0 && r < s.len() {
                if s[l - 1] == s[r] {
                    l -= 1;
                    r += 1;
                    res += 1;
                } else {
                    break;
                }
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
    fn palindrome_substrings() {
        assert_eq!(Solution::count_substrings("abc".into()), 3);
        assert_eq!(Solution::count_substrings("aaa".into()), 6);
    }
}
