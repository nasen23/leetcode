use std::collections::HashMap;

impl Solution {
    pub fn can_construct(s: String, k: i32) -> bool {
        if (s.as_bytes().len() as i32) < k {
            return false;
        }
        let counter = s.chars().fold(HashMap::new(), |mut counter, ch| {
            let count = counter.entry(ch).or_insert(0);
            *count += 1;
            counter
        });

        counter.values().map(|x| x % 2).sum::<i32>() <= k
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn can_construct() {
        assert!(Solution::can_construct("abcd".into(), 4));
        assert!(!Solution::can_construct("leetcode".into(), 2));
    }
}
