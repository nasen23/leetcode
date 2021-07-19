impl Solution {
    pub fn max_frequency(nums: Vec<i32>, k: i32) -> i32 {
        let mut nums = nums;
        nums.sort_unstable();
        let (mut i, mut j) = (0, 0);
        let mut res = 1;
        let mut sum = 0;
        while j < nums.len() {
            while nums[j] * (j - i) as i32 - sum > k {
                sum -= nums[i];
                i += 1;
            }
            sum += nums[j];
            res = res.max((j - i + 1) as i32);
            j += 1;
        }
        res
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn frequency_of_the_most_frequent_element() {
        assert_eq!(3, Solution::max_frequency(vec![1, 2, 4], 5));
        assert_eq!(2, Solution::max_frequency(vec![1, 4, 8, 13], 5));
        assert_eq!(1, Solution::max_frequency(vec![3, 6, 9], 2));
        assert_eq!(
            73,
            Solution::max_frequency(
                vec![
                    9930, 9923, 9983, 9997, 9934, 9952, 9945, 9914, 9985, 9982, 9970, 9932, 9985,
                    9902, 9975, 9990, 9922, 9990, 9994, 9937, 9996, 9964, 9943, 9963, 9911, 9925,
                    9935, 9945, 9933, 9916, 9930, 9938, 10000, 9916, 9911, 9959, 9957, 9907, 9913,
                    9916, 9993, 9930, 9975, 9924, 9988, 9923, 9910, 9925, 9977, 9981, 9927, 9930,
                    9927, 9925, 9923, 9904, 9928, 9928, 9986, 9903, 9985, 9954, 9938, 9911, 9952,
                    9974, 9926, 9920, 9972, 9983, 9973, 9917, 9995, 9973, 9977, 9947, 9936, 9975,
                    9954, 9932, 9964, 9972, 9935, 9946, 9966
                ],
                3056
            )
        );
    }
}
