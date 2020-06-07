use std::collections::VecDeque;

impl Solution {
    pub fn find_ladders(
        begin_word: String,
        end_word: String,
        word_list: Vec<String>,
    ) -> Vec<Vec<String>> {
        let mut edges = vec![vec![]; word_list.len() + 2];
        let mut word_list = word_list;
        if !word_list.iter().any(|s| s.eq(&end_word)) {
            return vec![];
        }
        word_list.push(begin_word);
        word_list.push(end_word);
        for i in 0..word_list.len() {
            for j in i + 1..word_list.len() {
                if diff_only_one(word_list[i].as_bytes(), word_list[j].as_bytes()) {
                    edges[i].push(j);
                    edges[j].push(i);
                }
            }
        }

        let mut cost = vec![std::usize::MAX; edges.len()];
        cost[edges.len() - 2] = 0;
        let mut queue = VecDeque::new();
        queue.push_back(vec![cost.len() - 2]);
        let mut res = vec![];
        while let Some(top) = queue.pop_front() {
            let last = *top.last().unwrap();
            if last == cost.len() - 1 {
                res.push(top.into_iter().map(|i| word_list[i].clone()).collect());
            } else {
                for &p in edges[last].iter() {
                    if cost[p] >= cost[last] + 1 {
                        cost[p] = cost[last] + 1;
                        let mut tmp = top.clone();
                        tmp.push(p);
                        queue.push_back(tmp);
                    }
                }
            }
        }

        res
    }
}

fn diff_only_one(s: &[u8], p: &[u8]) -> bool {
    let mut diff = false;
    for i in 0..s.len() {
        if s[i] != p[i] {
            if diff {
                return false;
            }
            diff = true;
        }
    }

    diff
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::vec_str;

    #[test]
    fn find_ladders() {
        assert_eq!(
            Solution::find_ladders(
                "hit".into(),
                "cog".into(),
                vec_str!["hot", "dot", "dog", "lot", "log", "cog"]
            ),
            vec![
                vec_str!["hit", "hot", "dot", "dog", "cog"],
                vec_str!["hit", "hot", "lot", "log", "cog"]
            ]
        );
    }
}
