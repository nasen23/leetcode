impl Solution {
    pub fn maximum_time(time: String) -> String {
        match time.as_bytes() {
            &[a, b, _, c, d] => {
                let maximize = |x: u8, y: u8, mx: u8, my: u8| {
                    let rx = if x == b'?' {
                        if y != b'?' && y > my {
                            mx - 1
                        } else {
                            mx
                        }
                    } else {
                        x
                    };
                    let ry = if y == b'?' {
                        if rx == mx {
                            my
                        } else {
                            b'9'
                        }
                    } else {
                        y
                    };
                    (rx, ry)
                };
                let (ra, rb) = maximize(a, b, b'2', b'3');
                let (rc, rd) = maximize(c, d, b'5', b'9');
                unsafe { std::str::from_utf8_unchecked(&[ra, rb, b':', rc, rd]).into() }
            }
            _ => unreachable!(),
        }
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn latest_time_by_replacing_hidden_digits() {
        assert_eq!("23:50", Solution::maximum_time("2?:?0".into()))
    }
}
