impl Solution {
    pub fn decode_string(s: String) -> String {
        decode_bytes_impl(s.as_bytes()).0
    }
}

fn decode_bytes_impl(bytes: &[u8]) -> (String, usize) {
    let mut s = String::new();
    let mut i = 0;
    while i < bytes.len() {
        match bytes[i] {
            b'0'..=b'9' => {
                let mut repeat = 0;
                while bytes[i] >= b'0' && bytes[i] <= b'9' {
                    repeat = repeat * 10 + bytes[i] - b'0';
                    i += 1;
                }
                i += 1;
                let (inner, len) = decode_bytes_impl(&bytes[i..]);
                s.push_str(&inner.repeat(repeat as usize));
                i += len;
            }
            b']' => break,
            _ => s.push(bytes[i] as char),
        }
        i += 1;
    }
    (s, i)
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn decode_string() {
        assert_eq!(Solution::decode_string("3[a]2[bc]".into()), "aaabcbc");
        assert_eq!(Solution::decode_string("3[a2[c]]".into()), "accaccacc");
        assert_eq!(
            Solution::decode_string("2[abc]3[cd]ef".into()),
            "abcabccdcdcdef"
        );
        assert_eq!(Solution::decode_string("10[a]".into()), "aaaaaaaaaa");
    }
}
