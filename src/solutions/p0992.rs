use std::collections::HashMap;

fn at_most_k(a: &[i32], k: i32) -> i32 {
    let mut ret = 0;
    let (mut i, mut j) = (0, 0);
    let mut counter: HashMap<i32, i32> = HashMap::new();
    let mut distinct = 0;
    while j < a.len() {
        let count = counter.entry(a[j]).or_default();
        if *count == 0 {
            distinct += 1;
        }
        *count += 1;
        while distinct > k {
            let count = counter.entry(a[i]).or_default();
            *count -= 1;
            if *count == 0 {
                distinct -= 1;
            }
            i += 1;
        }
        j += 1;
        ret += j - i;
    }
    ret as i32
}

impl Solution {
    pub fn subarrays_with_k_distinct(a: Vec<i32>, k: i32) -> i32 {
        at_most_k(&a, k) - at_most_k(&a, k - 1)
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn subarrays_with_k_different_integers() {
        assert_eq!(
            7,
            Solution::subarrays_with_k_distinct(vec![1, 2, 1, 2, 3], 2)
        );
    }
}
