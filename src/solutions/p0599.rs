use std::collections::HashMap;

impl Solution {
    pub fn find_restaurant(list1: Vec<String>, list2: Vec<String>) -> Vec<String> {
        let map = list1
            .into_iter()
            .enumerate()
            .map(|t| (t.1, t.0))
            .collect::<HashMap<_, _>>();
        let cons = list2
            .into_iter()
            .enumerate()
            .filter_map(|t| map.get(&t.1).map(|x| (t.1, x + t.0)))
            .collect::<Vec<_>>();
        let min = cons.iter().map(|t| t.1).min().unwrap();
        cons.into_iter()
            .filter(|t| t.1 == min)
            .map(|t| t.0)
            .collect()
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vec_str;

    #[test]
    fn minimum_index_sum_of_2_lists() {
        assert_eq!(
            Solution::find_restaurant(
                vec_str!["Shogun", "Tapioca Express", "Burger King", "KFC"],
                vec_str![
                    "Piatti",
                    "The Grill at Torrey Pines",
                    "Hungry Hunter Steakhouse",
                    "Shogun"
                ]
            ),
            vec_str!["Shogun"]
        );
        assert_eq!(
            Solution::find_restaurant(
                vec_str!["Shogun", "Tapioca Express", "Burger King", "KFC"],
                vec_str!["KFC", "Shogun", "Burger King"]
            ),
            vec_str!["Shogun"]
        );
    }
}
