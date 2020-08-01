use std::cmp::Reverse;
use std::collections::BinaryHeap;

impl Solution {
    pub fn smallest_range(nums: Vec<Vec<i32>>) -> Vec<i32> {
        let (mut l, mut r) = (0, std::i32::MAX);
        let mut max = nums.iter().map(|v| v[0]).max().unwrap();
        let mut heap = nums
            .iter()
            .enumerate()
            .map(|(i, v)| (Reverse(v[0]), i, 0))
            .collect::<BinaryHeap<_>>();
        loop {
            let (min, row, idx) = heap.pop().unwrap();
            let min = min.0;
            if max - min < r - l {
                l = min;
                r = max;
            }
            let idx = idx + 1;
            if idx == nums[row].len() {
                break vec![l, r];
            }
            max = max.max(nums[row][idx]);
            heap.push((Reverse(nums[row][idx]), row, idx));
        }
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vec_vec;

    #[test]
    fn smallest_range_covering_elems_from_lists() {
        assert_eq!(
            Solution::smallest_range(vec_vec![
                [4, 10, 15, 24, 26],
                [0, 9, 12, 20],
                [5, 18, 22, 30]
            ]),
            vec![20, 24]
        );
    }
}
