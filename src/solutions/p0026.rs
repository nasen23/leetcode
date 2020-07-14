impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        if nums.len() < 2 {
            return nums.len() as i32;
        }
        let mut j = 0;
        let mut i = 1;
        while i < nums.len() {
            if nums[j] != nums[i] {
                j += 1;
                nums[j] = nums[i];
            }
            i += 1;
        }
        (j + 1) as i32
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn remove_duplicates() {
        assert_eq!(
            Solution::remove_duplicates(&mut vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4]),
            5
        );
    }
}
