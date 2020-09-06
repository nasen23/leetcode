use std::collections::HashMap;

impl Solution {
    pub fn num_triplets(mut nums1: Vec<i32>, mut nums2: Vec<i32>) -> i32 {
        nums1.sort_unstable();
        nums2.sort_unstable();
        num_impl(&nums1, &nums2) + num_impl(&nums2, &nums1)
    }
}

fn num_impl(a: &[i32], b: &[i32]) -> i32 {
    let mut map = HashMap::new();
    for i in 0..a.len() - 1 {
        for j in i + 1..a.len() {
            *map.entry(a[i] as i64 * (a[j] as i64)).or_insert(0) += 1;
        }
    }
    let mut res = 0;
    for &n in b {
        let n = n as i64;
        if let Some(&x) = map.get(&(n * n)) {
            res += x;
        }
    }
    res
}

struct Solution;
