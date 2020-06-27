impl Solution {
    pub fn reorganize_string(s: String) -> String {
        let mut s = s;
        let mut chars = vec![(0, 0); 26];
        for ch in s.bytes() {
            chars[(ch - b'a') as usize].0 = ch;
            chars[(ch - b'a') as usize].1 += 1;
        }
        chars.sort_unstable_by(|a, b| b.1.cmp(&a.1));
        if chars[0].1 > (s.len() + 1) / 2 {
            return "".into();
        }
        let mut pos = 0;
        let bytes = unsafe { s.as_bytes_mut() };
        for (ch, count) in chars {
            if count == 0 {
                break;
            }
            for _ in 0..count {
                if pos >= bytes.len() {
                    pos = 1;
                }
                bytes[pos] = ch;
                pos += 2;
            }
        }
        s
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reorganize_string() {
        assert_eq!(Solution::reorganize_string("aab".into()), "aba");
        assert_eq!(Solution::reorganize_string("aabb".into()), "abab");
        assert_eq!(Solution::reorganize_string("aabbcc".into()), "abcabc");
    }
}
