use std::collections::VecDeque;

impl Solution {
    pub fn longest_subarray(nums: Vec<i32>, limit: i32) -> i32 {
        let (mut min, mut max) = (VecDeque::new(), VecDeque::new());
        let (mut i, mut j) = (0, 0);
        let mut res = 1;
        while j < nums.len() {
            while !min.is_empty() && *min.back().unwrap() > nums[j] {
                min.pop_back();
            }
            min.push_back(nums[j]);
            while !max.is_empty() && *max.back().unwrap() < nums[j] {
                max.pop_back();
            }
            max.push_back(nums[j]);
            j += 1;
            while *max.front().unwrap() - *min.front().unwrap() > limit {
                if nums[i] == *max.front().unwrap() {
                    max.pop_front();
                }
                if nums[i] == *min.front().unwrap() {
                    min.pop_front();
                }
                i += 1;
            }
            res = res.max(j - i);
        }
        res as i32
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn longest_subarray_with_diff_less_or_equal_to_limit() {
        assert_eq!(2, Solution::longest_subarray(vec![2, 4, 7], 4));
        assert_eq!(2, Solution::longest_subarray(vec![8, 2, 4, 7], 4));
        assert_eq!(4, Solution::longest_subarray(vec![10, 1, 2, 4, 7, 2], 5));
    }
}
