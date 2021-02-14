impl Solution {
    pub fn pivot_index(nums: Vec<i32>) -> i32 {
        let sum: i32 = nums.iter().sum();
        let mut l = 0;
        for i in 0..nums.len() {
            if l * 2 + nums[i] == sum {
                return i as i32;
            }
            l += nums[i];
        }
        -1
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_pivot_index() {
        assert_eq!(3, Solution::pivot_index(vec![1, 7, 3, 6, 5, 6]));
        assert_eq!(-1, Solution::pivot_index(vec![1, 2, 3]));
        assert_eq!(0, Solution::pivot_index(vec![2, 1, -1]));
    }
}
