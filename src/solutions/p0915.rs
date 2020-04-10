struct Solution;

impl Solution {
    pub fn partition_disjoint(a: Vec<i32>) -> i32 {
        let len = a.len();
        let mut min = vec![1000000; len];
        min[len - 1] = a[len - 1];
        for i in (0..len - 1).rev() {
            min[i] = std::cmp::min(min[i + 1], a[i]);
        }

        let mut max = 0;
        for (i, x) in a.into_iter().enumerate().take(len - 1) {
            max = std::cmp::max(max, x);
            if max <= min[i + 1] {
                return i as i32 + 1;
            }
        }

        len as i32
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test() {
        assert_eq!(Solution::partition_disjoint(vec![5, 0, 3, 8, 6]), 3);
        assert_eq!(Solution::partition_disjoint(vec![1, 1, 1, 0, 6, 12]), 4);
        assert_eq!(Solution::partition_disjoint(vec![4, 3, 2, 1]), 4);
    }
}
