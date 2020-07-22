impl Solution {
    pub fn min_array(nums: Vec<i32>) -> i32 {
        let (mut l, mut r) = (0, nums.len() - 1);
        while l < r {
            let m = (l + r) / 2;
            if nums[r] > nums[m] {
                r = m;
            } else if nums[r] < nums[m] {
                l = m + 1;
            } else {
                r -= 1;
            }
        }
        nums[l]
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn minimum_in_rotated_sorted_array() {
        assert_eq!(Solution::min_array(vec![3, 4, 5, 1, 2]), 1);
        assert_eq!(Solution::min_array(vec![3, 1]), 1);
    }
}
