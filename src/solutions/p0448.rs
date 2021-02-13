impl Solution {
    pub fn find_disappeared_numbers(mut nums: Vec<i32>) -> Vec<i32> {
        for i in 0..nums.len() {
            let idx = (nums[i].abs() - 1) as usize;
            if nums[idx] > 0 {
                nums[idx] = -nums[idx];
            }
        }
        nums.into_iter()
            .enumerate()
            .filter(|(_, x)| *x > 0)
            .map(|(i, _)| (i + 1) as i32)
            .collect()
    }
}

struct Solution;
