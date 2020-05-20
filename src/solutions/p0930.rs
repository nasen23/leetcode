use std::collections::HashMap;

impl Solution {
    pub fn num_subarrays_with_sum(a: Vec<i32>, s: i32) -> i32 {
        let mut map = HashMap::new();
        let mut sum = 0;
        let mut res = 0;
        map.insert(0, 1);
        for n in a {
            sum += n;
            if let Some(c) = map.get(&(sum - s)) {
                res += c;
            }
            let count = map.entry(sum).or_insert(0);
            *count += 1;
        }

        res
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn num_subarrays_with_sum() {
        assert_eq!(Solution::num_subarrays_with_sum(vec![1, 0, 1, 0, 1], 2), 4);
    }
}
