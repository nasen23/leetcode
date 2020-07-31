impl Solution {
    pub fn find_magic_index(nums: Vec<i32>) -> i32 {
        let mut i = 0;
        while i < nums.len() {
            if (i as i32) == nums[i] {
                return i as i32;
            }
            if (i as i32) < nums[i] {
                i = nums[i] as usize;
            } else {
                i += 1;
            }
        }
        -1
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn magic_index() {
        assert_eq!(Solution::find_magic_index(vec![0, 2, 3, 4, 5]), 0);
        assert_eq!(Solution::find_magic_index(vec![1, 1, 1]), 1);
    }
}
