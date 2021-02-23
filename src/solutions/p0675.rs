use std::collections::{BTreeMap, VecDeque};

fn min_step_bfs(forest: &Vec<Vec<i32>>, src: (usize, usize), dst: (usize, usize)) -> Option<i32> {
    let (m, n) = (forest.len(), forest[0].len());
    let mut visited = vec![vec![false; n]; m];
    let mut deque = VecDeque::new();
    deque.push_back(src);
    visited[src.0][src.1] = true;
    let mut step = 0;
    while !deque.is_empty() {
        for _ in 0..deque.len() {
            let p = deque.pop_front().unwrap();
            if p == dst {
                return Some(step);
            }
            for (i, j) in [
                p.0.checked_sub(1).map(|i| (i, p.1)),
                p.1.checked_sub(1).map(|j| (p.0, j)),
                if p.0 + 1 < m {
                    Some((p.0 + 1, p.1))
                } else {
                    None
                },
                if p.1 + 1 < n {
                    Some((p.0, p.1 + 1))
                } else {
                    None
                },
            ]
            .iter()
            .filter_map(|x| *x)
            .filter(|(i, j)| forest[*i][*j] > 0)
            {
                if visited[i][j] {
                    continue;
                }
                deque.push_back((i, j));
                visited[i][j] = true;
            }
        }
        step += 1;
    }
    None
}

impl Solution {
    pub fn cut_off_tree(forest: Vec<Vec<i32>>) -> i32 {
        let (m, n) = (forest.len(), forest[0].len());
        let mut map = BTreeMap::new();
        for i in 0..m {
            for j in 0..n {
                if forest[i][j] > 1 {
                    map.insert(forest[i][j], (i, j));
                }
            }
        }
        let mut last = (0, 0);
        let mut res = 0;
        for (_, pos) in map.into_iter() {
            match min_step_bfs(&forest, last, pos) {
                Some(step) => res += step,
                None => return -1,
            }
            last = pos;
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
    fn cut_off_trees_for_golf_event() {
        assert_eq!(
            6,
            Solution::cut_off_tree(vec_vec![[1, 2, 3], [0, 0, 4], [7, 6, 5]])
        );
        assert_eq!(
            -1,
            Solution::cut_off_tree(vec_vec![[1, 2, 3], [0, 0, 0], [7, 6, 5]])
        );
    }
}
