impl Solution {
    pub fn convert(s: String, num_rows: i32) -> String {
        if num_rows == 1 {
            return s;
        }
        let mut res = String::new();
        let s = s.as_bytes();
        let num_rows = num_rows as usize;
        let rem = 2 * num_rows - 2;
        for i in 0..num_rows {
            if i == 0 || i == num_rows - 1 {
                for j in 0.. {
                    if j * rem + i < s.len() {
                        res.push(s[j * rem + i] as char);
                    } else {
                        break;
                    }
                }
            } else {
                for j in 0.. {
                    if j * rem + i < s.len() {
                        res.push(s[j * rem + i] as char);
                    } else {
                        break;
                    }
                    if (j + 1) * rem - i < s.len() {
                        res.push(s[(j + 1) * rem - i] as char);
                    }
                }
            }
        }

        res
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn zigzag_conversion() {
        assert_eq!(Solution::convert("PLAY".into(), 2), "PALY");
        assert_eq!(
            Solution::convert("PAYPALISHIRING".into(), 3),
            "PAHNAPLSIIGYIR"
        );
        assert_eq!(
            Solution::convert("PAYPALISHIRING".into(), 4),
            "PINALSIGYAHRPI"
        );
    }
}
