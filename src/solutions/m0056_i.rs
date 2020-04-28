impl Solution {
    pub fn single_numbers(nums: Vec<i32>) -> Vec<i32> {
        let ret = nums.iter().fold(0, |acc, x| acc ^ x);
        let n = ret & -ret;
        let res = nums.iter().fold((0, 0), |(r1, r2), x| {
            if x & n == 0 {
                (r1 ^ x, r2)
            } else {
                (r1, r2 ^ x)
            }
        });

        vec![res.0, res.1]
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn single_numbers() {
        assert_eq!(
            {
                let mut vec = Solution::single_numbers(vec![4, 1, 4, 6]);
                vec.sort();
                vec
            },
            vec![1, 6]
        );
        assert_eq!(
            {
                let mut vec = Solution::single_numbers(vec![1, 2, 10, 4, 1, 4, 3, 3]);
                vec.sort();
                vec
            },
            vec![2, 10]
        );
    }
}
