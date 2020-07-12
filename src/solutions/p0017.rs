impl Solution {
    pub fn letter_combinations(digits: String) -> Vec<String> {
        if digits.is_empty() {
            return vec![];
        }
        let mut res = vec!["".into()];
        for digit in digits.chars() {
            res = res
                .into_iter()
                .map(|s| {
                    get_letters(digit)
                        .into_iter()
                        .map(move |l| format!("{}{}", s, l))
                })
                .flatten()
                .collect();
        }
        res
    }
}

fn get_letters(x: char) -> Vec<String> {
    match x {
        '2' => vec!["a".into(), "b".into(), "c".into()],
        '3' => vec!["d".into(), "e".into(), "f".into()],
        '4' => vec!["g".into(), "h".into(), "i".into()],
        '5' => vec!["j".into(), "k".into(), "l".into()],
        '6' => vec!["m".into(), "n".into(), "o".into()],
        '7' => vec!["p".into(), "q".into(), "r".into(), "s".into()],
        '8' => vec!["t".into(), "u".into(), "v".into()],
        '9' => vec!["w".into(), "x".into(), "y".into(), "z".into()],
        _ => unreachable!(),
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vec_str;

    #[test]
    fn letter_combine() {
        assert_eq!(
            Solution::letter_combinations("23".into()),
            vec_str!["ad", "ae", "af", "bd", "be", "bf", "cd", "ce", "cf"]
        );
    }
}
