use std::collections::VecDeque;

struct Solution;

impl Solution {
    pub fn find132pattern(nums: Vec<i32>) -> bool {
        let mut stack: VecDeque<i32> = VecDeque::with_capacity(nums.len());
        let mut num2 = std::i32::MIN;

        for &num in nums.iter().rev() {
            if num < num2 {
                return true;
            }
            while !stack.is_empty() && num > *stack.back().unwrap() {
                num2 = stack.pop_back().unwrap();
            }
            stack.push_back(num);
        }

        false
    }
}
