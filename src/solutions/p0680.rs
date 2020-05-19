impl Solution {
    pub fn valid_palindrome(s: String) -> bool {
        let bytes = s.as_bytes();
        if bytes.iter().eq(bytes.iter().rev()) {
            return true;
        }

        let (mut l, mut r) = (0, bytes.len() - 1);
        while l < r {
            if bytes[l] == bytes[r] {
                l += 1;
                r -= 1;
            } else {
                return valid_palindrome(bytes, l + 1, r) || valid_palindrome(bytes, l, r - 1);
            }
        }

        true
    }
}

fn valid_palindrome(bytes: &[u8], mut l: usize, mut r: usize) -> bool {
    while l < r {
        if bytes[l] != bytes[r] {
            return false;
        }
        l += 1;
        r -= 1;
    }
    true
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn valid_palindrome() {
        assert!(!Solution::valid_palindrome("abcdefg".into()));
        assert!(Solution::valid_palindrome("abba".into()));
        assert!(Solution::valid_palindrome("abbda".into()));
    }
}
