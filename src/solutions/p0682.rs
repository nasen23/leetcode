struct Solution;

use std::collections::VecDeque;

impl Solution {
    pub fn cal_points(ops: Vec<String>) -> i32 {
        let mut st: VecDeque<i32> = VecDeque::with_capacity(ops.len());

        for s in ops {
            match s.as_str() {
                "C" => {
                    st.pop_front();
                }
                "D" => {
                    let score = *st.front().unwrap();
                    st.push_front(2 * score);
                }
                "+" => {
                    let score = st.pop_front().unwrap();
                    let score2 = *st.front().unwrap();
                    st.push_front(score);
                    st.push_front(score + score2);
                }
                s => {
                    let score = s.parse::<i32>().unwrap();
                    st.push_front(score);
                }
            }
        }

        st.into_iter().sum()
    }
}

#[test]
fn case() {
    let ops = vec!["5", "2", "C", "D", "+"]
        .into_iter()
        .map(String::from)
        .collect();
    assert_eq!(Solution::cal_points(ops), 30);
}
