impl Solution {
    pub fn is_happy(n: i32) -> bool {
        let (mut s, mut f) = (n, n);
        let square_sum = |x: i32| {
            x.to_string()
                .bytes()
                .map(|by| ((by - b'0') as i32).pow(2))
                .sum()
        };
        loop {
            s = square_sum(s);
            f = square_sum(f);
            f = square_sum(f);
            if s == f {
                break;
            }
        }
        s == 1
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn happy_number() {
        assert!(Solution::is_happy(19));
        assert!(!Solution::is_happy(18));
    }
}
