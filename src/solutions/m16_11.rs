impl Solution {
    pub fn diving_board(shorter: i32, longer: i32, k: i32) -> Vec<i32> {
        if k == 0 {
            return vec![];
        }
        if shorter == longer {
            return vec![k * shorter];
        }
        (0..=k)
            .map(|i| (longer - shorter) * i + shorter * k)
            .collect()
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn diving_board() {
        assert_eq!(Solution::diving_board(1, 2, 3), vec![3, 4, 5, 6]);
    }
}
