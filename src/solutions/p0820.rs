struct Solution;

impl Solution {
    pub fn minimum_length_encoding(words: Vec<String>) -> i32 {
        let mut words: Vec<String> = words
            .into_iter()
            .map(|s| s.chars().rev().collect())
            .collect::<Vec<_>>();

        words.sort();

        let mut ans = 0;
        for (i, word) in words.iter().enumerate() {
            if i + 1 < words.len() && words[i + 1].starts_with(word) {
                continue;
            } else {
                ans += word.len() as i32 + 1;
            }
        }

        ans
    }
}

#[test]
fn case1() {
    let words = vec!["time", "me", "bell"];
    let words = words.into_iter().map(String::from).collect();
    assert_eq!(Solution::minimum_length_encoding(words), 10);
}

#[test]
fn case2() {
    let words = vec!["afterall", "all", "afterall"];
    let words = words.into_iter().map(String::from).collect();
    assert_eq!(Solution::minimum_length_encoding(words), 9);
}
