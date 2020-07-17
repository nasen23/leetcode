impl Solution {
    pub fn max_score(card_points: Vec<i32>, k: i32) -> i32 {
        let k = k as usize;
        let n = card_points.len();
        let sum = card_points.iter().sum();
        if n <= k {
            return sum;
        }
        let min: i32 = card_points
            .as_slice()
            .windows(n - k)
            .map(|v| v.into_iter().sum())
            .min()
            .unwrap();
        sum - min
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn maximum_score_from_cards() {
        assert_eq!(Solution::max_score(vec![1, 2, 3, 4, 5, 6, 1], 3), 12);
        assert_eq!(Solution::max_score(vec![2, 2, 2], 2), 4);
        assert_eq!(
            Solution::max_score(vec![1, 79, 80, 1, 1, 1, 200, 1], 3),
            202
        );
    }
}
