struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn contains_nearby_duplicate(nums: Vec<i32>, k: i32) -> bool {
        let mut map = HashMap::new();
        let mut max_dist = nums.len();

        for (i, n) in nums.into_iter().enumerate() {
            match map.get(&n) {
                Some(idx) => {
                    max_dist = std::cmp::min(max_dist, i - idx);
                    if max_dist as i32 <= k {
                        return true;
                    }
                    map.insert(n, i);
                }
                None => {
                    map.insert(n, i);
                }
            }
        }

        false
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test() {
        assert_eq!(
            Solution::contains_nearby_duplicate(vec![1, 2, 3, 1], 3),
            true
        );
        assert_eq!(
            Solution::contains_nearby_duplicate(vec![1, 0, 1, 1], 1),
            true
        );
        assert_eq!(
            Solution::contains_nearby_duplicate(vec![1, 2, 3, 1, 2, 3], 2),
            false
        );
    }
}
