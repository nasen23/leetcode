use std::collections::{HashMap, HashSet};

fn find_equation_dfs<'a>(
    graph: &HashMap<&String, HashSet<&'a String>>,
    values: &mut HashMap<(&String, &String), f64>,
    a: &'a String,
    b: &String,
    visited: &mut HashSet<&'a String>,
) -> Option<f64> {
    if !graph.contains_key(a) || !graph.contains_key(b) {
        return None;
    }
    if visited.contains(a) {
        return None;
    }
    visited.insert(a);
    if let Some(res) = values.get(&(a, b)) {
        return Some(*res);
    }
    for c in graph.get(a).unwrap() {
        if let Some(res) = find_equation_dfs(graph, values, c, b, visited) {
            return Some(res * values.get(&(a, c)).copied().unwrap());
        }
    }
    None
}

impl Solution {
    pub fn calc_equation(
        equations: Vec<Vec<String>>,
        values: Vec<f64>,
        queries: Vec<Vec<String>>,
    ) -> Vec<f64> {
        let mut map = HashMap::new();
        let mut graph = HashMap::new();
        for i in 0..equations.len() {
            let (e, val) = (&equations[i], values[i]);
            let s = graph.entry(&e[0]).or_insert(HashSet::new());
            s.insert(&e[1]);
            let s = graph.entry(&e[1]).or_insert(HashSet::new());
            s.insert(&e[0]);
            map.insert((&e[0], &e[1]), val);
            map.insert((&e[1], &e[0]), 1.0 / val);
        }
        queries
            .into_iter()
            .map(|query| {
                let (a, b) = (&query[0], &query[1]);
                let mut visited = HashSet::new();
                find_equation_dfs(&graph, &mut map, a, b, &mut visited).unwrap_or(-1.0)
            })
            .collect()
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vec_vec;

    #[test]
    fn evaluate_division() {
        assert_eq!(
            vec![6.00000, 0.50000, -1.00000, 1.00000, -1.00000],
            Solution::calc_equation(
                vec_vec![["a".into(), "b".into()], ["b".into(), "c".into()]],
                vec![2.0, 3.0],
                vec_vec![
                    ["a".into(), "c".into()],
                    ["b".into(), "a".into()],
                    ["a".into(), "e".into()],
                    ["a".into(), "a".into()],
                    ["x".into(), "x".into()]
                ]
            )
        );
    }
}
