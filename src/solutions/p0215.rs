struct Solution;

impl Solution {
    pub fn find_kth_largest(nums: Vec<i32>, k: i32) -> i32 {
        // just lazy now to write something
        let mut nums = nums;
        nums.sort();
        nums[nums.len() - k as usize]
    }
}

#[test]
fn test() {
    assert_eq!(Solution::find_kth_largest(vec![3, 2, 1, 5, 6, 4], 2), 5);
}
