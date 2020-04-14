use std::time::{SystemTime, UNIX_EPOCH};

struct Solution {
    nums: Vec<i32>,
}

impl Solution {
    fn new(nums: Vec<i32>) -> Self {
        Self { nums }
    }

    fn pick(&self, target: i32) -> i32 {
        // Xorshift RNGs
        let mut random = sys_time();
        let mut gen_u32 = || {
            random ^= random << 13;
            random ^= random >> 17;
            random ^= random << 5;
            random
        };

        self.nums
            .iter()
            .enumerate()
            .filter(|(_, &x)| x == target)
            .map(|(i, _)| i)
            .fold((0, 0), |(n, i), x| {
                let i = if gen_u32() % (n + 1) == 0 { x } else { i };
                (n + 1, i)
            })
            .1 as i32
    }
}

#[inline]
fn sys_time() -> u32 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .subsec_nanos()
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test() {
        let s = Solution::new(vec![1, 2, 3, 4, 4, 4, 4]);
        assert_eq!(s.pick(1), 0);
        assert_eq!(s.pick(2), 1);
    }
}
