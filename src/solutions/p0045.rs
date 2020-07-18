impl Solution {
    pub fn jump(nums: Vec<i32>) -> i32 {
        // maximizing the reached distance
        if nums.len() == 1 {
            return 0;
        }
        let (mut r, mut next) = (0, nums[0] as usize);
        let mut step = 0;
        for i in 0..nums.len() {
            next = next.max(i + nums[i] as usize);
            if next >= nums.len() - 1 {
                return step + 1;
            }
            if i == r {
                step += 1;
                r = next;
            }
        }
        step
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn jump_game2() {
        assert_eq!(Solution::jump(vec![2, 3, 1, 1, 4]), 2);
    }
}
