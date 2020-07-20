use std::collections::HashMap;

impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut anagrams = HashMap::new();
        for s in strs {
            let mut cnt = [0; 26];
            s.bytes().for_each(|c| cnt[(c - b'a') as usize] += 1);
            anagrams.entry(cnt).or_insert(Vec::new()).push(s);
        }
        anagrams.values().cloned().collect()
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vec_str;

    #[test]
    fn group_anagrams() {
        assert_eq!(
            Solution::group_anagrams(vec_str!["eat", "tea", "tan", "ate", "nat", "bat"]),
            vec![
                vec_str!["ate", "eat", "tea"],
                vec_str!["nat", "tan"],
                vec_str!["bat"]
            ]
        )
    }
}
