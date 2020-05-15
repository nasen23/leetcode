use std::collections::HashMap;

impl Solution {
    pub fn subarray_sum(nums: Vec<i32>, k: i32) -> i32 {
        let mut map = HashMap::with_capacity(nums.len());
        let mut sum = 0;
        let mut res = 0;
        map.insert(0, 1);
        for num in nums {
            sum += num;
            if let Some(count) = map.get(&(sum - k)) {
                res += count;
            }
            let count = map.entry(sum).or_insert(0);
            *count += 1;
        }

        res
    }

    pub fn subarray_fold(nums: Vec<i32>, k: i32) -> i32 {
        let len = nums.len();
        nums.into_iter()
            .fold(
                (
                    {
                        let mut map = HashMap::with_capacity(len);
                        map.insert(0, 1);
                        map
                    },
                    0,
                    0,
                ),
                |(mut map, mut res, mut sum), x| {
                    sum = sum + x;
                    if let Some(count) = map.get(&(sum - k)) {
                        res += count;
                    }
                    let count = map.entry(sum).or_insert(0);
                    *count += 1;
                    (map, res, sum)
                },
            )
            .1
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn subarray_sum() {
        assert_eq!(Solution::subarray_sum(vec![1, 1, 1], 2), 2);
        assert_eq!(Solution::subarray_sum(vec![1, 2, 3], 3), 2);
        assert_eq!(Solution::subarray_sum(vec![1, -1], -1), 1);
        assert_eq!(Solution::subarray_sum(vec![1, 2, 2, 1], 3), 2);
        assert_eq!(Solution::subarray_sum(vec![1, 2, 1, 2, 1], 3), 4);
    }
}

#[cfg(test)]
mod bench {
    extern crate test;
    use self::test::Bencher;
    use super::Solution;

    #[bench]
    fn subarray_sum_loop(b: &mut Bencher) {
        b.iter(|| Solution::subarray_sum(vec![1000; 10000], 1000));
    }

    #[bench]
    fn subarray_sum_fold(b: &mut Bencher) {
        b.iter(|| Solution::subarray_fold(vec![1000; 10000], 1000));
    }
}
