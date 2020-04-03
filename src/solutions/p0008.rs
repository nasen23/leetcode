struct Solution;

impl Solution {
    pub fn my_atoi(s: String) -> i32 {
        let mut minus = 1;
        let mut ans: i32 = 0;
        for (i, c) in s.trim().chars().enumerate() {
            let c: char = c;
            match c {
                '0'..='9' => match ans
                    .checked_mul(10)
                    .and_then(|x| x.checked_add(minus * c.to_digit(10).unwrap() as i32))
                {
                    Some(res) => ans = res,
                    None => {
                        return if minus == 1 {
                            i32::max_value()
                        } else {
                            i32::min_value()
                        }
                    }
                },
                '+' if i == 0 => {}
                '-' if i == 0 => minus = -1,
                _ => break,
            }
        }

        ans
    }
}

#[test]
fn test() {
    assert_eq!(Solution::my_atoi("42".into()), 42);
    assert_eq!(Solution::my_atoi("   -42".into()), -42);
    assert_eq!(Solution::my_atoi("4193 with words".into()), 4193);
    assert_eq!(Solution::my_atoi("-91283472332".into()), i32::min_value());
}
