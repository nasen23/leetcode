use std::cmp;

impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        assert!(height.len() > 1);
        let (mut i, mut j) = (0, height.len() - 1);
        let mut res = 0;
        while i < j {
            res = cmp::max(res, cmp::min(height[i], height[j]) * (j - i) as i32);
            if height[i] < height[j] {
                i += 1;
            } else {
                j -= 1;
            }
        }

        res
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn container_with_most_water() {
        assert_eq!(Solution::max_area(vec![1, 8, 6, 2, 5, 4, 8, 3, 7]), 49);
    }
}
