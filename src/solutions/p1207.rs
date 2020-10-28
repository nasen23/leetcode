impl Solution {
    pub fn unique_occurrences(arr: Vec<i32>) -> bool {
        let map = arr
            .iter()
            .fold(std::collections::HashMap::new(), |mut map, &x| {
                *map.entry(x).or_insert(0) += 1;
                map
            });
        let mut set = std::collections::HashSet::new();
        for (_, v) in map {
            if !set.insert(v) {
                return false;
            }
        }
        true
    }
}

struct Solution;
