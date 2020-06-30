use std::collections::VecDeque;

impl Solution {
    pub fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut res = vec![];
        let mut deque = VecDeque::new();
        for i in 0..nums.len() {
            // remove last
            while deque.back().map_or(false, |&x| nums[i] > nums[x]) {
                deque.pop_back();
            }
            deque.push_back(i);
            if deque.front().map(|&x| x + k as usize) == Some(i) {
                deque.pop_front();
            }
            if i + 1 >= k as usize {
                res.push(nums[*deque.front().unwrap()])
            }
        }
        res
    }

    pub fn max_sliding_window_dp(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let k = k as usize;
        if nums.len() * k == 0 {
            return vec![];
        }
        if k == 1 {
            return nums;
        }
        let mut left = vec![0; nums.len()];
        left[0] = nums[0];
        let mut right = vec![0; nums.len()];
        right[nums.len() - 1] = nums[nums.len() - 1];
        let mut res = vec![];
        for i in 1..nums.len() {
            if i % k == 0 {
                left[i] = nums[i];
            } else {
                left[i] = left[i - 1].max(nums[i]);
            }
            let j = nums.len() - 1 - i;
            if (j + 1) % k == 0 {
                right[j] = nums[j];
            } else {
                right[j] = right[j + 1].max(nums[j]);
            }
        }
        for i in 0..nums.len() - k + 1 {
            res.push(right[i].max(left[i + k - 1]));
        }
        res
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn max_sliding_window() {
        assert_eq!(
            Solution::max_sliding_window_dp(vec![1, 3, -1, -3, 5, 3, 6, 7], 3),
            vec![3, 3, 5, 5, 6, 7]
        )
    }
}
