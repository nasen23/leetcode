impl Solution {
    pub fn shortest_palindrome(s: String) -> String {
        let n = s.len();
        if n < 2 {
            return s;
        }
        let mut min = n - 1;
        for x in (1..n).rev() {
            let s = s.as_bytes();
            let (mut l, mut r) = ((x + 1) / 2, (x + 2) / 2);
            while l > 0 && r < n && s[l - 1] == s[r] {
                l -= 1;
                r += 1;
            }
            if l == 0 {
                min = min.min(n - r);
            }
        }
        let mut p = s.clone();
        let s = s.as_bytes();
        for i in n - min..n {
            p.insert(0, s[i] as char);
        }
        p
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shortest_palindrome() {
        assert_eq!(Solution::shortest_palindrome("abcd".into()), "dcbabcd");
        assert_eq!(Solution::shortest_palindrome("aacecaaa".into()), "aaacecaaa");
    }
}
