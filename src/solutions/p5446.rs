impl Solution {
    pub fn min_difference(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        if n <= 4 {
            return 0;
        }
        let mut nums = nums;
        nums.sort();
        (nums[n - 4] - nums[0])
            .min(nums[n - 3] - nums[1])
            .min(nums[n - 2] - nums[2])
            .min(nums[n - 1] - nums[3])
    }
}

struct Solution;
