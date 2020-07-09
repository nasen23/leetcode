use std::collections::HashSet;

const P: u64 = (1 << 31) - 1;
const BASE: u64 = 41;

fn hash(s: &str) -> u64 {
    let mut h = 0;
    for ch in s.bytes().rev() {
        h = (h * BASE + (ch - b'a') as u64 + 1) % P;
    }
    h
}

impl Solution {
    pub fn respace(dictionary: Vec<String>, sentence: String) -> i32 {
        let hashs = dictionary
            .into_iter()
            .map(|s| hash(&s))
            .collect::<HashSet<_>>();
        let sentence = sentence.as_bytes();
        let mut f = vec![0; sentence.len() + 1];
        for i in 1..=sentence.len() {
            f[i] = f[i - 1] + 1;
            let mut val = 0;
            for j in (0..i).rev() {
                let t = (sentence[j] - b'a' + 1) as u64;
                val = (val * BASE + t) % P;
                if hashs.contains(&val) {
                    f[i] = f[i].min(f[j]);
                }
            }
        }
        f[sentence.len()]
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vec_str;

    #[test]
    fn respace_lcci() {
        assert_eq!(Solution::respace(vec_str!["time"], "time".into()), 0);
        assert_eq!(Solution::respace(vec_str!["time"], "xtimex".into()), 2);
        assert_eq!(
            Solution::respace(
                vec_str!["looked", "just", "like", "her", "brother"],
                "jesslookedjustliketimherbrother".into()
            ),
            7
        );
        assert_eq!(
            Solution::respace(
                vec_str!["wccm", "wiw", "uwwiwcmiiiwmmwicuwu", "mw"],
                "iwiwwwmuiccwwwwwmumwwwmcciwwuiwcicwwuwicuiwciwmiwicwuwwmuimccwucuuim".into()
            ),
            63
        );
        assert_eq!(
            Solution::respace(
                vec_str![
                    "sssjjs",
                    "hschjf",
                    "hhh",
                    "fhjchfcfshhfjhs",
                    "sfh",
                    "jsf",
                    "cjschjfscscscsfjcjfcfcfh",
                    "hccccjjfchcffjjshccsjscsc",
                    "chcfjcsshjj",
                    "jh",
                    "h",
                    "f",
                    "s",
                    "jcshs",
                    "jfjssjhsscfc"
                ],
                "sssjjssfshscfjjshsjjsjchffffs".into()
            ),
            7
        );
    }
}
