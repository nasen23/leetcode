use std::collections::HashMap;

impl Solution {
    pub fn can_reorder_doubled(a: Vec<i32>) -> bool {
        let mut a = a;
        a.sort_unstable();
        let mut map = a.iter().fold(HashMap::new(), |mut map, x| {
            let count = map.entry(x).or_insert(0);
            *count += 1;
            map
        });
        for n in a.iter() {
            if *map.get(&n).unwrap() > 0 {
                if *n < 0 {
                    if n % 2 == 0 && map.get(&(n / 2)).map_or(false, |&x| x > 0) {
                        let c = map.get_mut(&(n / 2)).unwrap();
                        *c -= 1;
                    } else {
                        return false;
                    }
                } else {
                    if map.get(&(n * 2)).map_or(false, |&x| x > 0) {
                        let c = map.get_mut(&(n * 2)).unwrap();
                        *c -= 1;
                    } else {
                        return false;
                    }
                }
                map.entry(n).and_modify(|c| *c -= 1);
            }
        }

        true
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn can_reorder_doubled() {
        assert!(!Solution::can_reorder_doubled(vec![3, 1, 3, 6]));
        assert!(!Solution::can_reorder_doubled(vec![2, 1, 2, 6]));
        assert!(Solution::can_reorder_doubled(vec![4, -2, 2, -4]));
        assert!(!Solution::can_reorder_doubled(vec![1, 2, 4, 16, 8, 4]));
        assert!(Solution::can_reorder_doubled(vec![1, 1, 2, 2]));
        assert!(Solution::can_reorder_doubled(vec![
            -1, 4, 6, 8, -4, 6, -6, 3, -2, 3, -3, -8
        ]));
    }
}
