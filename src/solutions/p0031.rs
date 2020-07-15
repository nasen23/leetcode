impl Solution {
    pub fn next_permutation(nums: &mut Vec<i32>) {
        if nums.len() < 2 {
            return;
        }
        let mut i = nums.len() - 2;
        let all = loop {
            if nums[i] >= nums[i + 1] {
                i = match i.checked_sub(1) {
                    Some(i) => i,
                    None => break true,
                }
            } else {
                break false;
            }
        };
        if all {
            nums.reverse();
        } else {
            // NOTE: possible opt, using binary search here
            let (j, _) = nums
                .iter()
                .enumerate()
                .rev()
                .take(nums.len() - 1 - i)
                .filter(|(_, &x)| x > nums[i])
                .min_by_key(|t| t.1)
                .unwrap();
            nums.swap(i, j);
            nums[i + 1..].reverse();
        }
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn next_permutation() {
        let mut v = vec![1, 2, 3];
        Solution::next_permutation(&mut v);
        assert_eq!(v, vec![1, 3, 2]);
        let mut v = vec![1, 5, 4, 2, 3];
        Solution::next_permutation(&mut v);
        assert_eq!(v, vec![1, 5, 4, 3, 2]);
        let mut v = vec![2, 5, 4, 3, 1];
        Solution::next_permutation(&mut v);
        assert_eq!(v, vec![3, 1, 2, 4, 5]);
        let mut v = vec![3, 2, 2, 2];
        Solution::next_permutation(&mut v);
        assert_eq!(v, vec![2, 2, 2, 3]);
    }
}
