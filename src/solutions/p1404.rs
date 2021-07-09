impl Solution {
    pub fn num_steps(s: String) -> i32 {
        let mut s = s.into_bytes();
        let mut res = 0;
        while s.len() > 1 {
            if *s.last().unwrap() == b'0' {
                s.pop();
            } else {
                let mut i = s.len() - 1;
                let mut overflow = false;
                while s[i] == b'1' {
                    s[i] = b'0';
                    if i > 0 {
                        i -= 1;
                    } else {
                        overflow = true;
                        break;
                    }
                }
                if overflow {
                    s.insert(0, b'1');
                } else {
                    s[i] = b'1';
                }
            }
            res += 1;
        }
        res
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn num_of_steps_reduce_to_1_binary() {
        assert_eq!(6, Solution::num_steps("1101".into()));
    }
}
