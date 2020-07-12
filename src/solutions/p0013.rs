use std::collections::HashMap;

impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        let map = [
            ('I', 1),
            ('V', 5),
            ('X', 10),
            ('L', 50),
            ('C', 100),
            ('D', 500),
            ('M', 1000),
        ]
        .iter()
        .cloned()
        .collect::<HashMap<_, _>>();
        let mut last = 0;
        s.chars().rev().fold(0, |acc, c| {
            let n = *map.get(&c).unwrap();
            let acc = acc + if n < last { -n } else { n };
            last = n;
            acc
        })
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn roman_to_int() {
        assert_eq!(Solution::roman_to_int("IX".into()), 9);
    }
}
