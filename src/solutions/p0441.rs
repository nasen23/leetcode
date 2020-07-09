impl Solution {
    pub fn arrange_coins(n: i32) -> i32 {
        ((((1 + 8 * (n as u64)) as f64).sqrt() - 1.0) / 2.0) as i32
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn arranging_coins() {
        assert_eq!(Solution::arrange_coins(5), 2);
        assert_eq!(Solution::arrange_coins(8), 3);
        assert_eq!(Solution::arrange_coins(1804289383), 60070);
    }
}
