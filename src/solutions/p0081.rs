impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> bool {
        let (mut l, mut r) = (0, nums.len());
        while l < r {
            let mid = (l + r) / 2;
            if nums[mid] == target {
                return true;
            } else if nums[l] < nums[mid] {
                if nums[l] <= target && target < nums[mid] {
                    r = mid;
                } else {
                    l = mid + 1;
                }
            } else if nums[mid] < nums[r - 1] {
                if nums[mid] < target && target <= nums[r - 1] {
                    l = mid + 1;
                } else {
                    r = mid;
                }
            } else {
                if nums[l] == nums[mid] {
                    l += 1;
                } else {
                    r -= 1;
                }
            }
        }
        false
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn search_rotated_sorted_arr2() {
        assert!(Solution::search(vec![2, 5, 6, 0, 0, 1, 2], 0));
        assert!(!Solution::search(vec![], 0));
        assert!(Solution::search(vec![1], 1));
        assert!(Solution::search(vec![1, 3, 5], 1));
        assert!(Solution::search(vec![1, 3, 1, 1], 3));
        assert!(Solution::search(vec![1, 1, 1, 3, 1], 3));
        assert!(!Solution::search(vec![2, 5, 6, 0, 0, 1, 2], 3));
    }
}
