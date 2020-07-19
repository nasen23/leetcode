impl Solution {
    pub fn num_water_bottles(num_bottles: i32, num_exchange: i32) -> i32 {
        let (mut empty, mut full) = (0, num_bottles);
        let mut res = 0;
        loop {
            if full == 0 {
                break;
            }
            res += full;
            empty += full;
            full = 0;
            full += empty / num_exchange;
            empty %= num_exchange;
        }
        res
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn water_bottles() {
        assert_eq!(Solution::num_water_bottles(9, 3), 13);
        assert_eq!(Solution::num_water_bottles(15, 4), 19);
        assert_eq!(Solution::num_water_bottles(5, 5), 6);
        assert_eq!(Solution::num_water_bottles(2, 3), 2);
    }
}
