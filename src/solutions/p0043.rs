impl Solution {
    pub fn multiply(num1: String, num2: String) -> String {
        let mut tmp = vec![0; num1.len() + num2.len()];
        for (i, a) in num1.bytes().enumerate().rev() {
            let a = a - b'0';
            for (j, b) in num2.bytes().enumerate().rev() {
                let b = b - b'0';
                let s = tmp[i + j + 1] + a * b;
                tmp[i + j + 1] = s % 10;
                tmp[i + j] += s / 10;
            }
        }

        let mut tmp = tmp
            .into_iter()
            .skip_while(|&x| x == 0)
            .map(|x| x + b'0')
            .collect::<Vec<_>>();
        if tmp.is_empty() {
            tmp.push(b'0');
        }
        unsafe { String::from_utf8_unchecked(tmp) }
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn multiply_strings() {
        assert_eq!(Solution::multiply("10".into(), "100".into()), "1000");
        assert_eq!(
            Solution::multiply("3271973129738".into(), "74239847293".into()),
            "242910785498548397099234"
        );
    }
}
