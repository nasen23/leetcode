impl Solution {
    pub fn divide(dividend: i32, divisor: i32) -> i32 {
        if dividend == 0 {
            return 0;
        }
        if divisor == 1 {
            return dividend;
        }
        if divisor == -1 {
            dbg!(dividend, std::i32::MIN);
            if dividend != std::i32::MIN {
                return -dividend;
            } else {
                return std::i32::MAX;
            }
        }
        let sign = (dividend < 0) != (divisor < 0);
        let dividend = if dividend > 0 { -dividend } else { dividend };
        let divisor = if divisor > 0 { -divisor } else { divisor };
        let res = div(dividend, divisor);
        if sign {
            -res
        } else {
            res
        }
    }
}

fn div(a: i32, b: i32) -> i32 {
    if a > b {
        return 0;
    }
    let mut c = 1;
    let mut bb = b;
    while bb.checked_add(bb).map_or(false, |x| x >= a) {
        c = c << 1;
        bb <<= 1;
    }
    c + div(a - bb, b)
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn divide() {
        assert_eq!(Solution::divide(11, 5), 2);
        assert_eq!(Solution::divide(10, 5), 2);
        assert_eq!(Solution::divide(7, -3), -2);
        assert_eq!(Solution::divide(10, 3), 3);
        assert_eq!(Solution::divide(std::i32::MAX, 2), std::i32::MAX / 2);
        assert_eq!(Solution::divide(std::i32::MAX, 2), std::i32::MAX / 2);
        assert_eq!(Solution::divide(-2147483648, 2), -1073741824);
    }
}
