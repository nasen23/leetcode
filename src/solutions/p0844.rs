impl Solution {
    pub fn backspace_compare(s: String, t: String) -> bool {
        let s = s.as_bytes();
        let t = t.as_bytes();
        let (mut ss, mut tt) = (0, 0);
        let (mut i, mut j) = (s.len(), t.len());
        while i.checked_sub(1).is_some() || j.checked_sub(1).is_some() {
            while let Some(ii) = i.checked_sub(1) {
                if s[ii] == b'#' {
                    ss += 1;
                    i -= 1;
                } else if ss > 0 {
                    i -= 1;
                    ss -= 1;
                } else {
                    break;
                }
            }
            while let Some(jj) = j.checked_sub(1) {
                if t[jj] == b'#' {
                    tt += 1;
                    j -= 1;
                } else if tt > 0 {
                    tt -= 1;
                    j -= 1;
                } else {
                    break;
                }
            }
            if i > 0 && j > 0 {
                if s[i - 1] != t[j - 1] {
                    return false;
                }
            } else if i > 0 || j > 0 {
                return false;
            }
            if i > 0 {
                i -= 1;
            }
            if j > 0 {
                j -= 1;
            }
        }
        true
    }
}

struct Solution;
