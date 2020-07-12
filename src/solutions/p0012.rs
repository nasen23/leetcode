#[inline]
fn append(s: &mut String, x: i32, ch1: char, ch2: char, ch3: char) {
    match x {
        0 => {}
        1..=3 => {
            for _ in 0..x {
                s.push(ch1);
            }
        }
        4 => {
            s.push(ch1);
            s.push(ch2);
        }
        5 => s.push(ch2),
        6..=8 => {
            s.push(ch2);
            for _ in 0..x - 5 {
                s.push(ch1);
            }
        }
        9 => {
            s.push(ch1);
            s.push(ch3);
        }
        _ => unreachable!(),
    }
}

impl Solution {
    pub fn int_to_roman(num: i32) -> String {
        let mut res = String::new();
        let mut x = num;
        for _ in 0..x / 1000 {
            res.push('M');
        }
        x %= 1000;
        append(&mut res, x / 100, 'C', 'D', 'M');
        x %= 100;
        append(&mut res, x / 10, 'X', 'L', 'C');
        x %= 10;
        append(&mut res, x, 'I', 'V', 'X');
        res
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn integer_to_roman() {
        assert_eq!(Solution::int_to_roman(3), "III");
        assert_eq!(Solution::int_to_roman(4), "IV");
        assert_eq!(Solution::int_to_roman(9), "IX");
        assert_eq!(Solution::int_to_roman(1994), "MCMXCIV");
    }
}
