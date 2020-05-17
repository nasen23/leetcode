use std::cmp;

impl Solution {
    pub fn max_product(nums: Vec<i32>) -> i32 {
        let mut max = std::i32::MIN;
        let (mut p, mut q) = (1, 1);
        for num in nums {
            if num < 0 {
                let t = q;
                q = p;
                p = t;
            }
            p = cmp::max(p * num, num);
            q = cmp::min(q * num, num);

            max = cmp::max(max, p);
        }

        max
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn max_product() {
        assert_eq!(Solution::max_product(vec![2, 3, -2, 4]), 6);
        assert_eq!(Solution::max_product(vec![-2, 0, -1]), 0);
    }
}
