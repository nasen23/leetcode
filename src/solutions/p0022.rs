struct Solution;

impl Solution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        let mut res = Vec::with_capacity(2usize.pow(n as u32));
        fn solve(l: i32, r: i32, s: String, res: &mut Vec<String>) {
            if l == 0 && r == 0 {
                res.push(s);
                return;
            }

            if l > 0 {
                solve(l - 1, r, s.clone() + "(", res);
            }
            if r > l {
                solve(l, r - 1, s.clone() + ")", res);
            }
        }

        solve(n, n, String::from(""), &mut res);
        res
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::vec_str;

    #[test]
    fn test() {
        assert_eq!(
            Solution::generate_parenthesis(3),
            vec_str!["((()))", "(()())", "(())()", "()(())", "()()()"]
        );
    }
}
