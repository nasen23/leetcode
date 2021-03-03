impl Solution {
    pub fn count_bits(num: i32) -> Vec<i32> {
        let mut res = vec![0; num as usize + 1];
        for i in 1..=num as usize {
            res[i] = res[i & (i - 1)] + 1;
        }
        res
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn counting_bits() {
        assert_eq!(vec![0, 1, 1], Solution::count_bits(2));
        assert_eq!(vec![0, 1, 1, 2], Solution::count_bits(3));
        assert_eq!(vec![0, 1, 1, 2, 1, 2], Solution::count_bits(5));
    }
}
