use std::collections::{HashMap, HashSet};

impl Solution {
    pub fn max_freq(s: String, max_letters: i32, min_size: i32, _max_size: i32) -> i32 {
        let min_size = min_size as usize;
        let max_letters = max_letters as usize;
        let bytes = s.as_bytes();

        let mut map = HashMap::new();

        for i in 0..bytes.len() - min_size + 1 {
            let set = bytes[i..i + min_size].into_iter().collect::<HashSet<_>>();
            if set.len() <= max_letters {
                let count = map.entry(bytes[i..i + min_size].to_vec()).or_insert(0);
                *count += 1;
            }
        }

        map.values().max().cloned().unwrap_or(0)
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn max_freq() {
        assert_eq!(Solution::max_freq("aababcaab".into(), 2, 3, 4), 2);
    }
}
