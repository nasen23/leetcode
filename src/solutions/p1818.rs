const MOD: i32 = 1000000007;

impl Solution {
    pub fn min_absolute_sum_diff(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let diff = nums1
            .iter()
            .zip(nums2.iter())
            .map(|(&x, &y)| (x - y).abs())
            .collect::<Vec<_>>();
        let initial_sum: i32 = diff.iter().fold(0, |agg, &x| (agg + x) % MOD);

        let mut candidates = nums1;
        candidates.sort();
        (initial_sum
            - nums2
                .iter()
                .zip(diff.iter())
                .map(|(&y, &d)| match candidates.binary_search(&y) {
                    Ok(_) => d,
                    Err(i) => i
                        .checked_sub(1)
                        .and_then(|j| candidates.get(j))
                        .map(|x| d - (x - y).abs())
                        .max(candidates.get(i).map(|x| d - (x - y).abs()))
                        .unwrap(),
                })
                .max()
                .unwrap())
        .rem_euclid(MOD)
    }
}

struct Solution;
