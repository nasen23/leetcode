impl Solution {
    pub fn modify_string(mut s: String) -> String {
        let by = unsafe { s.as_bytes_mut() };
        for i in 0..by.len() {
            if by[i] == b'?' {
                for x in b'a'..=b'z' {
                    if (i == 0 || x != by[i - 1]) && (i == by.len() - 1 || x != by[i + 1]) {
                        by[i] = x;
                    }
                }
            }
        }
        s
    }
}

struct Solution;
