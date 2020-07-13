use std::collections::HashMap;

impl Solution {
    pub fn intersect(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let mut map = nums2.into_iter().fold(HashMap::new(), |mut map, x| {
            let c = map.entry(x).or_insert(0);
            *c += 1;
            map
        });
        nums1.into_iter().fold(vec![], |mut v, x| {
            let c = map.get_mut(&x);
            if let Some(c) = c {
                if *c > 0 {
                    v.push(x);
                    *c -= 1;
                }
            }
            v
        })
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn intersection_of_2_arrays_2() {
        assert_eq!(
            Solution::intersect(vec![1, 2, 2, 1], vec![2, 2]),
            vec![2, 2]
        );
        assert_eq!(
            Solution::intersect(vec![4, 9, 5], vec![9, 4, 9, 8, 4]),
            vec![4, 9]
        );
    }
}
