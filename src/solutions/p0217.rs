use std::collections::HashSet;

impl Solution {
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        let mut set = HashSet::new();
        for n in nums {
            if !set.insert(n) {
                return true;
            }
        }

        false
    }
}

struct Solution;
