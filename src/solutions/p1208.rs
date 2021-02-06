impl Solution {
    pub fn equal_substring(s: String, t: String, max_cost: i32) -> i32 {
        let (s, t) = (s.as_bytes(), t.as_bytes());
        let (mut i, mut j) = (0, 0);
        let mut cost = 0;
        while j < s.len() {
            cost += (s[j] as i32 - t[j] as i32).abs();
            j += 1;
            if cost > max_cost {
                cost -= (s[i] as i32 - t[i] as i32).abs();
                i += 1;
            }
        }
        (s.len() - i) as i32
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_equal_substrings_within_budget() {
        assert_eq!(
            3,
            Solution::equal_substring("abcd".into(), "bcdf".into(), 3)
        );
        assert_eq!(
            1,
            Solution::equal_substring("abcd".into(), "cdef".into(), 3)
        );
        assert_eq!(
            1,
            Solution::equal_substring("abcd".into(), "acde".into(), 0)
        );
        assert_eq!(
            1,
            Solution::equal_substring("abad".into(), "bcae".into(), 0)
        );
    }
}
