impl Solution {
    pub fn sort_colors(nums: &mut Vec<i32>) {
        let (mut i, mut j) = (0, nums.len());
        let mut k = 0;
        while k < j {
            match nums[k] {
                0 => {
                    nums.swap(i, k);
                    i += 1;
                    k += 1;
                }
                2 if j > 0 => {
                    nums.swap(k, j - 1);
                    j -= 1;
                }
                _ => k += 1,
            }
        }
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sort_colors() {
        let mut nums = vec![2, 0, 2, 1, 1, 0];
        Solution::sort_colors(&mut nums);
        assert_eq!(nums, vec![0, 0, 1, 1, 2, 2]);
        let mut nums = vec![2];
        Solution::sort_colors(&mut nums);
        assert_eq!(nums, vec![2]);
    }
}
