impl Solution {
    pub fn range_bitwise_and(mut m: i32, mut n: i32) -> i32 {
        let mut i = 0;
        while m != n {
            m >>= 1;
            n >>= 1;
            i += 1;
        }
        n << i
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn range_bitwise_and() {
        assert_eq!(Solution::range_bitwise_and(5, 7), 4);
    }
}
