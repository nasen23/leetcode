struct Solution;

impl Solution {
    pub fn sort_array(nums: Vec<i32>) -> Vec<i32> {
        let mut nums = nums;
        let last = nums.len() - 1;
        quick_sort(&mut nums, 0, last);
        nums
    }
}

fn quick_sort(nums: &mut Vec<i32>, first: usize, last: usize) {
    if first < last {
        let mid = partition(nums, first, last);
        if let Some(res) = mid.checked_sub(1) {
            quick_sort(nums, first, res);
        }
        quick_sort(nums, mid + 1, last);
    }
}

fn partition(nums: &mut Vec<i32>, first: usize, last: usize) -> usize {
    let pivot = nums[last];

    let mut i = first;

    for j in first..=last - 1 {
        if nums[j] < pivot {
            nums.swap(i, j);
            i += 1;
        }
    }

    nums.swap(i, last);
    i
}

#[test]
fn case1() {
    let nums = vec![5, 3, 2, 1];
    assert_eq!(Solution::sort_array(nums), vec![1, 2, 3, 5]);
    let nums = vec![5, 1, 1, 2, 0, 0];
    assert_eq!(Solution::sort_array(nums), vec![0, 0, 1, 1, 2, 5]);
    let nums = vec![1, 2, 3, 4];
    assert_eq!(Solution::sort_array(nums), vec![1, 2, 3, 4]);
    let nums = (0..4).rev().collect();
    assert_eq!(Solution::sort_array(nums), vec![0, 1, 2, 3]);
    // let nums: Vec<i32> = (0..50000).collect();
    // assert_eq!(Solution::sort_array(nums), (0..50000).collect::<Vec<_>>());
}
