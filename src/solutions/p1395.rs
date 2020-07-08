impl Solution {
    pub fn num_teams(rating: Vec<i32>) -> i32 {
        let mut res = 0;
        for j in 1..rating.len() - 1 {
            let (mut il, mut ih) = (0, 0);
            let (mut kl, mut kh) = (0, 0);
            for i in 0..j {
                if rating[i] < rating[j] {
                    il += 1;
                } else {
                    ih += 1;
                }
            }
            for k in j + 1..rating.len() {
                if rating[k] < rating[j] {
                    kl += 1;
                } else {
                    kh += 1;
                }
            }
            res += il * kh + ih * kl;
        }
        res
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn count_number_of_teams() {
        assert_eq!(Solution::num_teams(vec![2, 5, 3, 4, 1]), 3);
        assert_eq!(Solution::num_teams(vec![2, 1, 3]), 0);
        assert_eq!(Solution::num_teams(vec![1, 2, 3, 4]), 4);
    }
}
