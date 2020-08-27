use std::collections::HashMap;

impl Solution {
    pub fn find_itinerary(tickets: Vec<Vec<String>>) -> Vec<String> {
        let mut map = tickets.iter().fold(HashMap::new(), |mut map, v| {
            let val = map.entry(v[0].clone()).or_insert(vec![]);
            let pos = val.binary_search_by(|a| v[1].cmp(a)).unwrap_or_else(|e| e);
            val.insert(pos, v[1].clone());
            map
        });
        let mut res = vec![];
        find_impl("JFK", &mut map, &mut res);
        res.reverse();
        res
    }
}

fn find_impl(start: &str, map: &mut HashMap<String, Vec<String>>, res: &mut Vec<String>) {
    while let Some(s) = map.get_mut(start).unwrap_or(&mut vec![]).pop() {
        find_impl(&s, map, res);
    }
    res.push(start.to_owned());
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{vec_str, vec_vec};

    #[test]
    fn find_itinerary() {
        assert_eq!(
            Solution::find_itinerary(
                vec_vec![
                    ["MUC", "LHR"],
                    ["JFK", "MUC"],
                    ["SFO", "SJC"],
                    ["LHR", "SFO"]
                ]
                .into_iter()
                .map(|v| v.into_iter().map(|s| s.to_owned()).collect())
                .collect()
            ),
            vec_str!["JFK", "MUC", "LHR", "SFO", "SJC"]
        );
        assert_eq!(
            Solution::find_itinerary(
                vec_vec![
                    ["JFK", "SFO"],
                    ["JFK", "ATL"],
                    ["SFO", "ATL"],
                    ["ATL", "JFK"],
                    ["ATL", "SFO"]
                ]
                .into_iter()
                .map(|v| v.into_iter().map(|s| s.to_owned()).collect())
                .collect()
            ),
            vec_str!["JFK","ATL","JFK","SFO","ATL","SFO"]
        );
    }
}
