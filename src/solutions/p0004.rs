impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let m = nums1.len();
        let n = nums2.len();
        let one = (m + n + 1) / 2;
        let two = (m + n + 2) / 2;

        (find_kth(&nums1, 0, &nums2, 0, one) + find_kth(&nums1, 0, &nums2, 0, two)) as f64 / 2.0
    }
}

fn find_kth(nums1: &[i32], i: usize, nums2: &[i32], j: usize, k: usize) -> i32 {
    if i >= nums1.len() {
        return nums2[j + k - 1];
    }
    if j >= nums2.len() {
        return nums1[i + k - 1];
    }
    if k == 1 {
        return std::cmp::min(nums1[i], nums2[j]);
    }
    let mid1 = if i + k / 2 - 1 < nums1.len() {
        nums1[i + k / 2 - 1]
    } else {
        std::i32::MAX
    };
    let mid2 = if j + k / 2 - 1 < nums2.len() {
        nums2[j + k / 2 - 1]
    } else {
        std::i32::MAX
    };
    if mid1 < mid2 {
        find_kth(nums1, i + k / 2, nums2, j, k - k / 2)
    } else {
        find_kth(nums1, i, nums2, j + k / 2, k - k / 2)
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn find_median_sorted_arrays() {
        assert_eq!(
            Solution::find_median_sorted_arrays(vec![1, 2], vec![3]),
            2.0
        );
    }
}
