struct Solution;

impl Solution {
    pub fn last_remaining(n: i32, m: i32) -> i32 {
        let mut nums: Vec<_> = (0..n).collect();

        let mut next = 0;
        while nums.len() > 1 {
            next = (next + m as usize - 1) % nums.len();
            nums.remove(next);
        }

        nums[0]
    }
}

#[test]
fn case1() {
    assert_eq!(Solution::last_remaining(5, 3), 3);
    assert_eq!(Solution::last_remaining(10, 17), 2);
}
