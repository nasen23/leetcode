use std::collections::HashMap;

impl Solution {
    pub fn number_of_subarrays(nums: Vec<i32>, k: i32) -> i32 {
        let nums: Vec<_> = nums.into_iter().map(|x| x % 2).collect();

        nums.into_iter()
            .fold(
                (
                    {
                        let mut map = HashMap::new();
                        map.insert(0, 1);
                        map
                    },
                    0,
                    0,
                ),
                |(mut map, mut res, mut sum), x| {
                    sum += x;
                    res += map.get(&(sum - k)).cloned().unwrap_or(0);
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
    fn count_number_of_nice_subarrays() {
        assert_eq!(Solution::number_of_subarrays(vec![1, 1, 2, 1, 1], 3), 2);
        assert_eq!(Solution::number_of_subarrays(vec![1, 1, 1, 1, 1], 1), 5);
        assert_eq!(Solution::number_of_subarrays(vec![2, 4, 6], 1), 0);
        assert_eq!(
            Solution::number_of_subarrays(vec![2, 2, 2, 1, 2, 2, 1, 2, 2, 2], 2),
            16
        );
    }
}
