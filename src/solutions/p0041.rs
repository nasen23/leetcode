impl Solution {
    pub fn first_missing_positive(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        for i in 0..nums.len() {
            loop {
                let x = nums[i] as usize;
                if x > 0 && x - 1 != i && x < nums.len() && x != nums[x - 1] as usize {
                    nums.swap(x - 1, i);
                } else {
                    break;
                }
            }
        }
        for (i, &x) in nums.iter().enumerate() {
            if i + 1 != x as usize {
                return (i + 1) as i32;
            }
        }
        nums.len() as i32 + 1
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_missing_positive() {
        assert_eq!(Solution::first_missing_positive(vec![-1, 0, 1]), 2);
        assert_eq!(Solution::first_missing_positive(vec![0, 0, 1]), 2);
        assert_eq!(Solution::first_missing_positive(vec![3, 4, -1, 1]), 2);
        assert_eq!(Solution::first_missing_positive(vec![2, 1]), 3);
        assert_eq!(Solution::first_missing_positive(vec![1, 1]), 2);
    }
}
