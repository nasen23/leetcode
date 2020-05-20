impl Solution {
    pub fn find_the_longest_substring(s: String) -> i32 {
        let mut status = 0;
        let mut res = 0;
        let mut pos = vec![None; 1 << 5];
        pos[0] = Some(0);
        for (i, ch) in s.bytes().enumerate() {
            match ch {
                b'a' => status ^= 1 << 0,
                b'e' => status ^= 1 << 1,
                b'i' => status ^= 1 << 2,
                b'o' => status ^= 1 << 3,
                b'u' => status ^= 1 << 4,
                _ => {}
            }
            if let Some(p) = pos[status as usize] {
                res = std::cmp::max(res, i + 1 - p);
            } else {
                pos[status as usize] = Some(i + 1);
            }
        }

        res as i32
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn find_the_longest_substring() {
        assert_eq!(Solution::find_the_longest_substring("leetcode".into()), 5);
    }
}
