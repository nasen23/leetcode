use std::collections::HashMap;

fn is_palindrome(s: &str) -> bool {
    s.chars().rev().eq(s.chars())
}

impl Solution {
    pub fn palindrome_pairs(words: Vec<String>) -> Vec<Vec<i32>> {
        let mut res = vec![];
        let words = words
            .into_iter()
            .enumerate()
            .map(|(i, s)| (s, i as i32))
            .collect::<HashMap<_, _>>();
        if let Some(i) = words.get("") {
            for (word, j) in &words {
                if i != j && is_palindrome(word) {
                    res.push(vec![*i, *j]);
                    res.push(vec![*j, *i]);
                }
            }
        }
        for (word, i) in &words {
            let rev = word.chars().rev().collect::<String>();
            if let Some(j) = words.get(&rev) {
                if i != j {
                    res.push(vec![*i, *j]);
                }
            }
        }
        for (word, i) in &words {
            for k in 1..word.len() {
                let (l, r) = word.split_at(k);
                if let Some(j) = words.get(&r.chars().rev().collect::<String>()) {
                    if is_palindrome(l) {
                        res.push(vec![*j, *i]);
                    }
                }
                if let Some(j) = words.get(&l.chars().rev().collect::<String>()) {
                    if is_palindrome(r) {
                        res.push(vec![*i, *j]);
                    }
                }
            }
        }
        res
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{vec_str, vec_vec};

    #[test]
    fn palindrome_pairs() {
        assert_eq!(
            Solution::palindrome_pairs(vec_str!["a", "abc", "aba", ""]),
            vec_vec![[0, 3], [3, 0], [2, 3], [3, 2]]
        );
        assert_eq!(
            Solution::palindrome_pairs(vec_str!["abcd", "dcba", "lls", "s", "sssll"]),
            vec_vec![[0, 1], [1, 0], [3, 2], [2, 4]]
        );
    }
}
