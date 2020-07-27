impl Solution {
    pub fn consecutive_numbers_sum(n: i32) -> i32 {
        (2..)
            .take_while(|&i| i * (i + 1) <= 2 * n)
            .filter(|&i| (n - i * (i + 1) / 2) % i == 0)
            .count() as i32
            + 1
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cons_numbers_sum() {
        assert_eq!(Solution::consecutive_numbers_sum(15), 4);
        assert_eq!(Solution::consecutive_numbers_sum(9), 3);
        assert_eq!(Solution::consecutive_numbers_sum(5), 2);
    }
}
