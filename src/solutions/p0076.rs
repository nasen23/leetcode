use std::collections::HashMap;

impl Solution {
    pub fn min_window(s: String, t: String) -> String {
        let s_bytes = s.as_bytes();
        let t_bytes = t.as_bytes();
        let s_len = s_bytes.len();
        let mut t_len = t_bytes.len();
        if s_len < t_len {
            return "".into();
        }
        if t_len == 0 {
            return s;
        }

        let mutate_map = |mut map: HashMap<u8, i32>, &b| {
            let count = map.entry(b).or_insert(0);
            *count -= 1;
            map
        };
        let mut map = t_bytes.into_iter().fold(HashMap::new(), mutate_map);
        let mut i = 0;
        let mut min = (0, std::usize::MAX);
        for (j, b) in s_bytes.iter().enumerate() {
            let count = map.entry(*b).or_insert(0);
            if *count < 0 {
                t_len -= 1;
            }
            *count += 1;
            if t_len == 0 {
                loop {
                    let count = map.get_mut(&s_bytes[i]).unwrap();
                    if *count > 0 {
                        *count -= 1;
                        i += 1;
                    } else {
                        break;
                    }
                }
                if j - i < min.1 - min.0 {
                    min = (i, j);
                }
                *map.get_mut(&s_bytes[i]).unwrap() -= 1;
                t_len += 1;
                i += 1;
            }
        }

        if min.1 == std::usize::MAX {
            return "".into();
        }
        std::str::from_utf8(&s_bytes[min.0..min.1 + 1])
            .unwrap()
            .into()
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn min_window() {
        assert_eq!(Solution::min_window("abcd".into(), "ab".into()), "ab");
        assert_eq!(
            Solution::min_window("ADOBECODEBANC".into(), "ABC".into()),
            "BANC"
        );
        assert_eq!(
            Solution::min_window("ADOBECODEBANC".into(), "ABC".into()),
            "BANC"
        );
    }
}
