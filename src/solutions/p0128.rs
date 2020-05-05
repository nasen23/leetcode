use std::collections::HashSet;

impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        let mut set: HashSet<i32> = nums.iter().cloned().collect();

        nums.into_iter().fold(0, |max, x| {
            if set.remove(&x) {
                let len = (1..).take_while(|i| set.remove(&(x - i))).count()
                    + (1..).take_while(|i| set.remove(&(x + i))).count()
                    + 1;
                std::cmp::max(len as i32, max)
            } else {
                max
            }
        })
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn longest_consecutive() {
        assert_eq!(Solution::longest_consecutive(vec![100, 4, 200, 1, 3, 2]), 4);
    }
}
