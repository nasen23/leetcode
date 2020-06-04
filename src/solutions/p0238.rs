impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let (mut left, mut right) = (1, 1);
        let mut res = vec![1; nums.len()];
        for (i, &n) in nums.iter().enumerate() {
            res[i] *= left;
            left *= n;
        }
        for (i, &n) in nums.iter().enumerate().rev() {
            res[i] *= right;
            right *= n;
        }

        res
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn product_except_self() {
        assert_eq!(
            Solution::product_except_self(vec![1, 2, 3, 4]),
            vec![24, 12, 8, 6]
        );
    }
}
