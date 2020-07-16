impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let (mut l, mut r) = (0, nums.len());
        while l < r {
            let mid = (l + r) / 2;
            if nums[mid] == target {
                return mid as i32;
            } else if nums[mid] <= nums[r - 1] {
                if nums[mid] < target && nums[r - 1] >= target {
                    l = mid + 1;
                } else {
                    r = mid;
                }
            } else {
                if nums[l] <= target && target < nums[mid] {
                    r = mid;
                } else {
                    l = mid + 1;
                }
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
    fn search_in_rotated_sorted_array() {
        assert_eq!(Solution::search(vec![4, 5, 6, 7, 0, 1, 2], 0), 4);
        assert_eq!(Solution::search(vec![3, 1], 3), 0);
        assert_eq!(Solution::search(vec![1, 3], 3), 1);
        assert_eq!(Solution::search(vec![1, 3, 5], 5), 2);
        assert_eq!(Solution::search(vec![3, 5, 1], 3), 0);
    }
}
