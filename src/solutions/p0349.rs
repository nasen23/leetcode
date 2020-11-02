use std::collections::HashSet;

impl Solution {
    pub fn intersection(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let s1 = nums1.into_iter().collect::<HashSet<_>>();
        let s2 = nums2.into_iter().collect::<HashSet<_>>();
        s1.intersection(&s2).cloned().collect()
    }
}

struct Solution;
