use std::collections::HashSet;

impl Solution {
    pub fn find_subsequences(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut res = vec![];
        dfs(&mut res, &mut vec![], &nums, 0);
        res
    }
}

fn dfs(res: &mut Vec<Vec<i32>>, seq: &mut Vec<i32>, nums: &Vec<i32>, p: usize) {
    if seq.len() > 1 {
        res.push(seq.clone());
    }
    let mut set = HashSet::new();
    for i in p..nums.len() {
        if set.insert(nums[i]) && seq.last().map_or(true, |&x| x <= nums[i]) {
            seq.push(nums[i]);
            dfs(res, seq, nums, i + 1);
            seq.pop();
        }
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vec_vec;

    #[test]
    fn find_subsequences() {
        assert_eq!(
            Solution::find_subsequences(vec![4, 6, 7, 7]),
            vec_vec![
                [4, 6],
                [4, 7],
                [4, 6, 7],
                [4, 6, 7, 7],
                [6, 7],
                [6, 7, 7],
                [7, 7],
                [4, 7, 7]
            ]
        );
    }
}
