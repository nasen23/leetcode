use std::collections::HashMap;

impl Solution {
    pub fn find_substring(s: String, words: Vec<String>) -> Vec<i32> {
        let s = s.as_bytes();
        if s.len() == 0 || words.len() == 0 {
            return vec![];
        }
        let len = words[0].len();
        let num = words.len();
        let all = num * len;
        if all > s.len() {
            return vec![];
        }
        let map = words.into_iter().fold(HashMap::new(), |mut map, s| {
            let c = map.entry(s.into_bytes()).or_insert(0);
            *c += 1;
            map
        });
        let mut res = vec![];
        for i in 0..len {
            let (mut l, mut r) = (i, i);
            let mut c = 0;
            let mut pmap = HashMap::new();
            while r + len <= s.len() {
                let w = &s[r..r + len];
                r += len;
                if let Some(&tc) = map.get(w) {
                    let pc = pmap.entry(w).or_insert(0);
                    *pc += 1;
                    c += 1;
                    while *pmap.get(w).unwrap() > tc {
                        c -= 1;
                        pmap.get_mut(&s[l..l + len]).map(|c| *c -= 1);
                        l += len;
                    }
                    if c == num {
                        res.push(l as i32);
                    }
                } else {
                    c = 0;
                    l = r;
                    pmap.clear();
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
    use crate::vec_str;

    #[test]
    fn substring_with_concatenation_of_all_words() {
        assert_eq!(
            Solution::find_substring("barfoothefoobarman".into(), vec_str!("foo", "bar")),
            vec![0, 9]
        );
    }
}
