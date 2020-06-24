impl Solution {
    pub fn three_sum_closest(nums: Vec<i32>, target: i32) -> i32 {
        let mut nums = nums;
        nums.sort();
        let mut closest = nums[0] + nums[1] + nums[2];
        for i in 0..nums.len() - 2 {
            let (mut l, mut r) = (i + 1, nums.len() - 1);
            while l < r {
                let sum = nums[i] + nums[l] + nums[r];
                if (sum - target).abs() < (closest - target).abs() {
                    closest = sum;
                }
                if sum > target {
                    r -= 1;
                } else if sum < target {
                    l += 1;
                } else {
                    return target;
                }
            }
        }

        closest
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn three_sum_closest() {
        assert_eq!(Solution::three_sum_closest(vec![-1, 2, 1, -4], 1), 2);
        assert_eq!(Solution::three_sum_closest(vec![1, 2, 3, 4, 5], 12), 12);
        assert_eq!(Solution::three_sum_closest(vec![3, 4, 5, 2, 1], 12), 12);
        assert_eq!(Solution::three_sum_closest(vec![2, 5, 4, 2, 3], 12), 12);
        assert_eq!(
            Solution::three_sum_closest(vec![1, 2, 4, 8, 16, 32, 64, 128], 82),
            82
        );
    }
}
