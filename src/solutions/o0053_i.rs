fn lower_bound(nums: &[i32], target: i32) -> usize {
    let (mut l, mut r) = (0, nums.len());
    while l < r {
        let mid = l + (r - l) / 2;
        use std::cmp::Ordering::*;
        match target.cmp(&nums[mid]) {
            Less | Equal => r = mid,
            Greater => l = mid + 1,
        }
    }
    l
}

fn upper_bound(nums: &[i32], target: i32) -> usize {
    let (mut l, mut r) = (0, nums.len());
    while l < r {
        let mid = l + (r - l) / 2;
        use std::cmp::Ordering::*;
        match target.cmp(&nums[mid]) {
            Greater | Equal => l = mid + 1,
            Less => r = mid,
        }
    }
    l
}

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let (i, j) = (lower_bound(&nums, target), upper_bound(&nums, target));
        (j - i) as i32
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn search_number_in_sorted_array() {
        assert_eq!(2, Solution::search(vec![5, 7, 7, 8, 8, 10], 8));
        assert_eq!(0, Solution::search(vec![5, 7, 7, 10], 8));
    }
}
