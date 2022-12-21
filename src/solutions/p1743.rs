use std::collections::HashMap;

impl Solution {
    pub fn restore_array(adjacent_pairs: Vec<Vec<i32>>) -> Vec<i32> {
        let mut map = HashMap::new();
        for pair in adjacent_pairs {
            let (a, b) = (pair[0], pair[1]);
            let insert_or_update = |map: &mut HashMap<_, _>, a, b| {
                let ent = map.entry(a).or_insert((b, None));
                if ent.0 != b {
                    ent.1 = Some(b);
                }
            };
            insert_or_update(&mut map, a, b);
            insert_or_update(&mut map, b, a);
        }

        let (mut prev, mut it) = map
            .iter()
            .find(|e| e.1 .1.is_none())
            .map(|e| (*e.0, e.1 .0))
            .unwrap();
        let mut res = vec![prev];
        loop {
            res.push(it);
            let (l, r) = map[&it];
            let next = if l == prev { r } else { Some(l) };
            if let Some(next) = next {
                prev = it;
                it = next;
            } else {
                break;
            }
        }
        res
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vec_vec;

    #[test]
    fn restore_array_from_adjacent_pairs() {
        assert_eq!(
            vec![1, 2, 3, 4],
            Solution::restore_array(vec_vec![[2, 1], [3, 4], [3, 2]])
        );
    }
}
