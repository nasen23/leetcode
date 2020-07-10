impl Solution {
    pub fn bag_of_tokens_score(tokens: Vec<i32>, p: i32) -> i32 {
        if tokens.is_empty() {
            return 0;
        }
        let (mut l, mut r) = (0, tokens.len() - 1);
        let mut tokens = tokens;
        tokens.sort();
        let mut p = p;
        let mut res = 0;
        while l <= r && p > 0 {
            while p > 0 && l <= r && p >= tokens[l] {
                p -= tokens[l];
                l += 1;
                res += 1;
            }
            if res > 0 && l < r {
                p += tokens[r];
                res -= 1;
                r -= 1;
            } else {
                return res;
            }
        }
        res
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bag_of_tokens() {
        assert_eq!(
            Solution::bag_of_tokens_score(vec![100, 200, 300, 400], 200),
            2
        );
    }
}
