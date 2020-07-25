impl Solution {
    pub fn num_of_subarrays(arr: Vec<i32>) -> i32 {
        let n = arr.len();
        let (odd, _) = arr.into_iter().map(|x| x % 2).enumerate().fold(
            (vec![0; n + 1], vec![0; n + 1]),
            |(mut odd, mut even), (i, x)| {
                if x == 0 {
                    odd[i + 1] = odd[i];
                    even[i + 1] = 1 + even[i];
                } else {
                    even[i + 1] = odd[i];
                    odd[i + 1] = 1 + even[i];
                }
                (odd, even)
            },
        );
        odd.into_iter()
            .fold(0, |acc, x| (acc + x) % (10i32.pow(9) + 7))
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn subarr_with_odd_sum() {
        assert_eq!(Solution::num_of_subarrays(vec![1, 3, 5]), 4);
        assert_eq!(Solution::num_of_subarrays(vec![2, 4, 6]), 0);
        assert_eq!(Solution::num_of_subarrays(vec![1, 2, 3, 4, 5, 6, 7]), 16);
        assert_eq!(Solution::num_of_subarrays(vec![100, 100, 99, 99]), 4);
        assert_eq!(Solution::num_of_subarrays(vec![7]), 1);
    }
}
