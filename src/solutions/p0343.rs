impl Solution {
    pub fn integer_break(n: i32) -> i32 {
        match n {
            2 => 1,
            3 => 2,
            mut n => {
                let mut a = 1;
                while n > 4 {
                    n -= 3;
                    a *= 3;
                }
                a * n
            }
        }
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn integer_break() {
        assert_eq!(Solution::integer_break(2), 1);
        assert_eq!(Solution::integer_break(10), 36);
    }
}
