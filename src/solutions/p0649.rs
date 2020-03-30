use std::collections::VecDeque;

struct Solution;

impl Solution {
    pub fn predict_party_victory(senate: String) -> String {
        let n = senate.len();
        let mut radiant = VecDeque::with_capacity(n);
        let mut dire = VecDeque::with_capacity(n);

        for (i, b) in senate.as_bytes().iter().enumerate() {
            if *b == b'R' {
                radiant.push_back(i);
            } else {
                dire.push_back(i);
            }
        }

        while !radiant.is_empty() && !dire.is_empty() {
            let r = radiant.pop_front().unwrap();
            let d = dire.pop_front().unwrap();

            if r < d {
                radiant.push_back(n + r);
            } else {
                dire.push_back(n + d);
            }
        }

        if radiant.is_empty() {
            "Dire"
        } else {
            "Radiant"
        }
        .into()
    }
}

#[test]
fn case1() {
    assert_eq!(Solution::predict_party_victory("RD".into()), "Radiant");
    assert_eq!(Solution::predict_party_victory("RDD".into()), "Dire");
    assert_eq!(Solution::predict_party_victory("RDDR".into()), "Radiant");
    assert_eq!(Solution::predict_party_victory("RRDDD".into()), "Radiant");
}
