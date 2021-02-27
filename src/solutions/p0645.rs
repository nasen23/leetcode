use std::collections::HashSet;

impl Solution {
    pub fn find_error_nums(nums: Vec<i32>) -> Vec<i32> {
        let mut res = vec![0, 0];
        let set = nums.into_iter().fold(HashSet::new(), |mut set, x| {
            if !set.insert(x) {
                res[0] = x;
            }
            set
        });
        for i in 1..=set.len() as i32 {
            if !set.contains(&i) {
                res[1] = i;
                break;
            }
        }
        res
    }
}

struct Solution;
