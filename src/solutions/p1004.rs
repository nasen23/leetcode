impl Solution {
    pub fn longest_ones(a: Vec<i32>, mut k: i32) -> i32 {
        let (mut i, mut j) = (0, 0);
        let mut res = 0;
        while j < a.len() {
            if a[j] == 0 {
                if k == 0 {
                    while a[i] == 1 {
                        i += 1;
                    }
                    i += 1;
                } else {
                    k -= 1;
                }
            }
            j += 1;
            res = res.max(j - i);
        }
        res as i32
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn max_consecutive_ones_iii() {
        assert_eq!(
            6,
            Solution::longest_ones(vec![1, 1, 1, 0, 0, 0, 1, 1, 1, 1, 0], 2)
        );
        assert_eq!(
            10,
            Solution::longest_ones(
                vec![0, 0, 1, 1, 0, 0, 1, 1, 1, 0, 1, 1, 0, 0, 0, 1, 1, 1, 1],
                3
            )
        );
    }
}
