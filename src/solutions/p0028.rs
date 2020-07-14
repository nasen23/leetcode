impl Solution {
    pub fn str_str(haystack: String, needle: String) -> i32 {
        // REVIEW: kmp
        let (s, p) = (haystack.as_bytes(), needle.as_bytes());
        let mut next = vec![0; p.len() + 1];
        next[0] = -1;
        let (mut i, mut j) = (0, -1);
        while i < p.len() {
            if j == -1 || p[i] == p[j as usize] {
                i += 1;
                j += 1;
                next[i] = j;
            } else {
                j = next[j as usize];
            }
        }
        let (mut i, mut j) = (0, 0);
        while i < s.len() && j < p.len() as i32 {
            if j == -1 || s[i] == p[j as usize] {
                i += 1;
                j += 1;
            } else {
                j = next[j as usize];
            }
        }
        if j == p.len() as i32 {
            i as i32 - j
        } else {
            -1
        }
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn implement_str_str() {
        assert_eq!(Solution::str_str("hello".into(), "ll".into()), 2);
        assert_eq!(Solution::str_str("hell".into(), "ll".into()), 2);
        assert_eq!(Solution::str_str("".into(), "".into()), 0);
    }
}
