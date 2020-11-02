use std::collections::HashMap;

impl Solution {
    pub fn word_break(s: String, word_dict: Vec<String>) -> Vec<String> {
        let mut res = HashMap::new();
        word_break_impl(&s, &word_dict, &mut res)
    }
}

fn word_break_impl(
    s: &str,
    dict: &Vec<String>,
    res: &mut HashMap<String, Vec<String>>,
) -> Vec<String> {
    if s.is_empty() {
        return vec!["".into()];
    }
    if let Some(v) = res.get(s) {
        return v.clone();
    }
    let mut tmp = vec![];
    for word in dict {
        if !s.starts_with(word){
            continue;
        }
        let ss = &s[word.len()..];
        let rr = word_break_impl(ss, dict, res);
        for ss in rr {
            tmp.push(if ss.is_empty() {
                word.clone()
            } else {
                format!("{} {}", word, ss)
            });
        }
    }
    res.insert(s.to_string(), tmp.clone());
    tmp
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vec_str;

    #[test]
    fn word_break_impl() {
        assert_eq!(Solution::word_break("bb".into(), vec_str!["a", "b", "bbb", "bbbb"]), vec_str!["b b"])
    }
}
