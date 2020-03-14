struct Solution;

impl Solution {
    pub fn length_of_lis(nums: Vec<i32>) -> i32 {
        let mut max = Vec::with_capacity(nums.len());
        for (i, x) in nums.iter().enumerate() {
            let next = max
                .iter()
                .take(i)
                .enumerate()
                .map(|(j, y)| if nums[j] < *x { *y + 1 } else { 1 })
                .max()
                .unwrap_or(1);

            max.push(next);
        }

        max.into_iter().max().unwrap_or(0)
    }
}

#[test]
fn case1() {
    let nums = vec![10, 9, 2, 5, 3, 7, 101, 18];
    assert_eq!(Solution::length_of_lis(nums), 4);
}

#[test]
fn case2() {
    let nums = vec![4, 10, 4, 3, 8, 9];
    assert_eq!(Solution::length_of_lis(nums), 3);
}

#[test]
fn case3() {
    let nums = vec![1, 3, 6, 7, 9, 4, 10, 5, 6];
    assert_eq!(Solution::length_of_lis(nums), 6);
}
