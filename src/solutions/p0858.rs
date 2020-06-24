impl Solution {
    pub fn mirror_reflection(p: i32, q: i32) -> i32 {
        let g = gcd(p, q);
        let (x, y) = (p / g, q / g);
        match (x % 2, y % 2) {
            (0, 1) => 2,
            (1, 0) => 0,
            (1, 1) => 1,
            _ => unreachable!(),
        }
    }
}

fn gcd(p: i32, q: i32) -> i32 {
    if p % q == 0 {
        q
    } else {
        gcd(q, p % q)
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mirror_reflection() {
        assert_eq!(Solution::mirror_reflection(2, 1), 2);
        assert_eq!(Solution::mirror_reflection(3, 1), 1);
        assert_eq!(Solution::mirror_reflection(5, 2), 0);
    }
}
