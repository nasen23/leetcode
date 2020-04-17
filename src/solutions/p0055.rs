use std::cmp;

impl Solution {
    pub fn can_jump(nums: Vec<i32>) -> bool {
        let len = nums.len();
        nums.into_iter().enumerate().fold(
            {
                let mut ve = vec![false; len];
                ve[0] = true;
                ve
            },
            |mut ve, (i, x)| {
                if ve[i] {
                    for j in i..cmp::min(i + 1 + x as usize, len) {
                        ve[j] = true;
                    }
                }
                ve
            },
        )[len - 1]
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn jump_game() {
        assert_eq!(Solution::can_jump(vec![2, 3, 1, 1, 4]), true);
        assert_eq!(Solution::can_jump(vec![3, 2, 1, 0, 4]), false);
    }
}
