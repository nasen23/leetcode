impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        let bytes = s.into_bytes();

        let mut max = (0, 0, 0);
        for i in 0..(2 * bytes.len()).checked_sub(1).unwrap_or(0) {
            let st = i / 2;
            let (mut l, mut r, mut len) = if i % 2 == 0 {
                (st as isize - 1, st + 1, 1)
            } else {
                (st as isize, st + 1, 0)
            };
            while l >= 0 && r < bytes.len() {
                if bytes[l as usize] == bytes[r] {
                    l -= 1;
                    r += 1;
                    len += 2;
                } else {
                    break;
                }
            }
            if len > max.0 {
                max = (len, (l + 1) as usize, r);
            }
        }

        if max.0 == 0 {
            "".into()
        } else {
            std::str::from_utf8(&bytes[max.1..max.2]).unwrap().into()
        }
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn longest_palindrome() {
        assert_eq!(Solution::longest_palindrome("abbcc".into()), "bb");
        assert_eq!(Solution::longest_palindrome("babad".into()), "bab");
        assert_eq!(Solution::longest_palindrome("cbbd".into()), "bb");
        assert_eq!(Solution::longest_palindrome("".into()), "");
        assert_eq!(Solution::longest_palindrome("a".into()), "a");
    }
}
