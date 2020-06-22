use std::collections::HashMap;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map = HashMap::new();
        for (i, x) in nums.into_iter().enumerate() {
            if let Some(&idx) = map.get(&(target - x)) {
                return vec![i as i32, idx];
            } else {
                map.insert(x, i as i32);
            }
        }

        unreachable!()
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn two_sum() {
        assert_eq!(Solution::two_sum(vec![2, 7, 11, 15], 9), vec![1, 0]);
    }
}
