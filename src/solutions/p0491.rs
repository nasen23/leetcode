use std::collections::HashSet;

impl Solution {
    pub fn find_subsequences(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let dp = nums.iter().map(|&x| vec![vec![x]]).collect::<Vec<_>>();
        for i in 1..nums.len() {
            for j in 0..i {
                for seq in dp[j].iter() {
                    if nums[i] >= *seq.last().unwrap() {
                        let mut seq = seq.clone();
                        seq.push(nums[i]);
                        unsafe {
                            (*(dp.as_ptr().add(i) as *mut Vec<Vec<i32>>)).push(seq);
                        }
                    }
                }
            }
        }

        let set = dp
            .into_iter()
            .skip(1)
            .flatten()
            .filter(|v| v.len() > 1)
            .collect::<HashSet<_>>();
        set.into_iter().collect()
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
