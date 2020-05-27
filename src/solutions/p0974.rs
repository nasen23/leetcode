impl Solution {
    pub fn subarrays_div_by_k(a: Vec<i32>, k: i32) -> i32 {
        let mut sum = 0;
        let mut res = 0;
        let mut map = vec![0; k as usize];
        map[0] = 1;
        for x in a {
            sum += x;
            let count = map.get_mut(sum.rem_euclid(k) as usize).unwrap();
            res += *count;
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
    fn subarrays_div_by_k() {
        assert_eq!(Solution::subarrays_div_by_k(vec![4, 5, 0, -2, -3, 1], 5), 7);
        assert_eq!(Solution::subarrays_div_by_k(vec![-1, 2, 9], 2), 2);
        assert_eq!(Solution::subarrays_div_by_k(vec![2, -2, 2, -4], 6), 2);
    }
}
