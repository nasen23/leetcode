impl Solution {
    pub fn minimum_size(nums: Vec<i32>, max_operations: i32) -> i32 {
        let (mut l, mut h) = (1, *nums.iter().max().unwrap());
        while l < h {
            let mid = (l + h) / 2;
            let mut op = 0;
            for &num in &nums {
                op += (num - 1) / mid;
            }
            if op <= max_operations {
                h = mid;
            } else {
                l = mid + 1;
            }
        }
        l
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn minimum_limit_of_balls_in_a_bag() {
        assert_eq!(3, Solution::minimum_size(vec![9], 2));
        assert_eq!(2, Solution::minimum_size(vec![2, 4, 8, 2], 4));
        assert_eq!(6, Solution::minimum_size(vec![7, 17], 3));
    }
}
