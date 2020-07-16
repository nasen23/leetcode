impl Solution {
    pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut res = vec![-1, -1];
        if nums.is_empty() {
            return res;
        }
        let (mut l, mut r) = (0, nums.len());
        while l < r {
            let m = (l + r - 1) / 2;
            if nums[m] >= target {
                r = m;
            } else {
                l = m + 1;
            }
        }
        if nums.get(l) != Some(&target) {
            return res;
        } else {
            res[0] = l as i32;
        }
        r = nums.len();
        while l < r {
            let m = (l + r) / 2;
            if nums[m] > target {
                r = m;
            } else {
                l = m + 1;
            }
        }
        res[1] = l as i32 - 1;
        res
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_first_last_pos_of_item_in_sorted_array() {
        assert_eq!(
            Solution::search_range(vec![5, 7, 7, 8, 8, 10], 8),
            vec![3, 4]
        );
        assert_eq!(
            Solution::search_range(vec![5, 7, 7, 8, 8, 10], 6),
            vec![-1, -1]
        );
        assert_eq!(Solution::search_range(vec![1], 1), vec![0, 0]);
        assert_eq!(Solution::search_range(vec![2, 2], 3), vec![-1, -1]);
    }
}
