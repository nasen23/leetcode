impl Solution {
    pub fn judge_circle(moves: String) -> bool {
        let (l, r, u, d) = moves
            .chars()
            .fold((0, 0, 0, 0), |(l, r, u, d), ch| match ch {
                'L' => (l + 1, r, u, d),
                'R' => (l, r + 1, u, d),
                'U' => (l, r, u + 1, d),
                'D' => (l, r, u, d + 1),
                _ => unreachable!(),
            });
        l == r && u == d
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn judge_circle() {
        assert!(Solution::judge_circle("UDUD".into()))
    }
}
