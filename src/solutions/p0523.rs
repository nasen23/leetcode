use std::collections::HashMap;

impl Solution {
    pub fn check_subarray_sum(nums: Vec<i32>, k: i32) -> bool {
        if nums.len() < 2 {
            return false;
        }
        for i in 1..nums.len() {
            if nums[i - 1] == 0 && nums[i] == 0 {
                return true;
            }
        }
        let mut k = k;
        if k == 0 {
            return false;
        } else if k < 0 {
            k = -k;
        }

        let mut map = HashMap::new();
        let mut sum = 0;
        for (i, num) in nums.into_iter().enumerate() {
            sum += num;
            let m = sum % k;
            if m == 0 && i > 0 {
                return true;
            }
            if let Some(prev) = map.get(&m) {
                if i - prev > 1 {
                    return true;
                }
            } else {
                map.insert(m, i);
            }
        }

        false
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn check_subarray_sum() {
        assert!(Solution::check_subarray_sum(vec![23, 2, 4, 6, 7], 6));
        assert!(Solution::check_subarray_sum(vec![23, 2, 6, 4, 7], 6));
        assert!(!Solution::check_subarray_sum(vec![23, 2, 6, 2, 5], 6));
        assert!(Solution::check_subarray_sum(vec![1, 2, 3, 4], 10));
        assert!(!Solution::check_subarray_sum(vec![1, 2, 3, 4], 11));
        assert!(!Solution::check_subarray_sum(
            vec![0, 1, 0, 3, 0, 4, 0, 4, 0],
            5
        ));
    }
}
