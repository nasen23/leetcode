impl Solution {
    pub fn divisor_game(n: i32) -> bool {
        n % 2 == 0
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn divisor_game() {
        assert!(Solution::divisor_game(2));
    }
}
