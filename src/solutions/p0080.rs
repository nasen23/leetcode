impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let mut i = 0;
        for j in 0..nums.len() {
            if i < 2 || nums[j] != nums[i - 2] {
                nums[i] = nums[j];
                i += 1;
            }
        }
        i as i32
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn remove_dup_from_sorted_arr2() {
        assert_eq!(Solution::remove_duplicates(&mut vec![1, 1, 1, 2, 2, 3]), 5);
        assert_eq!(
            Solution::remove_duplicates(&mut vec![0, 0, 1, 1, 1, 1, 2, 3, 3]),
            7
        );
    }
}
