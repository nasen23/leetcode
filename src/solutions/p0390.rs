struct Solution;

impl Solution {
    pub fn last_remaining(n: i32) -> i32 {
        match n {
            1 => 1,
            2 | 3 => 2,
            n if n % 4 < 2 => 4 * Solution::last_remaining(n / 4) - 2,
            n if n % 4 > 1 => 4 * Solution::last_remaining(n / 4),
            _ => unreachable!(),
        }
    }
}

#[test]
fn case1() {
    assert_eq!(Solution::last_remaining(9), 6);
}
