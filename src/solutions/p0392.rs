impl Solution {
    pub fn is_subsequence(s: String, t: String) -> bool {
        if s.len() > t.len() {
            return false;
        }
        let mut f = vec![vec![None; 26]; t.len() + 1];
        for (i, ch) in t.bytes().enumerate().rev() {
            if i < t.len() - 1 {
                for j in 0..26 {
                    f[i][j] = f[i + 1][j];
                }
            }
            f[i][(ch - b'a') as usize] = Some(i);
        }
        let mut cur = 0;
        for b in s.bytes() {
            if let Some(i) = f[cur][(b - b'a') as usize] {
                cur = i + 1;
            } else {
                return false;
            }
        }
        true
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_subseq() {
        assert_eq!(
            Solution::is_subsequence("abc".into(), "ahbgdc".into()),
            true
        );
        assert_eq!(
            Solution::is_subsequence("axc".into(), "ahbgdc".into()),
            false
        );
    }
}
