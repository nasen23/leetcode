use std::collections::HashMap;

impl Solution {
    pub fn find_shortest_sub_array(nums: Vec<i32>) -> i32 {
        let mut map = HashMap::new();
        let mut degree = 0;
        for (i, &x) in nums.iter().enumerate() {
            let v = map.entry(x).or_insert((0, i, i));
            v.0 += 1;
            v.2 = i;
            degree = degree.max(v.0);
        }
        map.into_iter()
            .map(|(_, v)| v)
            .filter(|v| v.0 == degree)
            .map(|v| v.2 - v.1 + 1)
            .min()
            .unwrap() as i32
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn degree_of_an_array() {
        assert_eq!(2, Solution::find_shortest_sub_array(vec![1, 2, 2, 3, 1]));
        assert_eq!(3, Solution::find_shortest_sub_array(vec![1, 1, 2, 2, 2, 1]));
        assert_eq!(
            6,
            Solution::find_shortest_sub_array(vec![1, 2, 2, 3, 1, 4, 2])
        );
    }
}
