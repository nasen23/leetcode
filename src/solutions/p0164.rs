impl Solution {
    pub fn maximum_gap(nums: Vec<i32>) -> i32 {
        if nums.len() < 2 {
            return 0;
        }
        let min = *nums.iter().min().unwrap();
        let max = *nums.iter().max().unwrap();
        let size = 1.max((max - min) as usize / (nums.len() - 1));
        let num = (max - min) as usize / size + 1;
        let mut buckets = vec![(false, std::i32::MAX, std::i32::MIN); num];
        for x in nums {
            let i = (x - min) as usize / size;
            buckets[i].0 = true;
            buckets[i].1 = x.min(buckets[i].1);
            buckets[i].2 = x.max(buckets[i].2);
        }
        let (mut prev, mut res) = (min, 0);
        for (_, min, max) in buckets.into_iter().filter(|b| b.0) {
            res = res.max(min - prev);
            prev = max;
        }
        res
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn maximum_gap() {
        assert_eq!(Solution::maximum_gap(vec![3, 6, 9, 1]), 3);
    }
}
