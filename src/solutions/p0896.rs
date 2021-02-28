use std::cmp::Ordering::*;

impl Solution {
    pub fn is_monotonic(a: Vec<i32>) -> bool {
        let mut ordering = Equal;
        for i in 1..a.len() {
            let o = a[i - 1].cmp(&a[i]);
            if o == Equal {
                continue;
            }
            if o != ordering {
                if ordering == Equal {
                    ordering = o;
                } else {
                    return false;
                }
            }
        }
        true
    }
}

struct Solution;
