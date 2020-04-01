struct Solution;

use std::collections::VecDeque;

impl Solution {
    pub fn max_depth_after_split(seq: String) -> Vec<i32> {
        let seq = seq.as_bytes();
        let mut ve = vec![0; seq.len()];
        let mut stack = VecDeque::with_capacity(seq.len());

        let mut i = 0;
        let mut max_d = 0;
        while i < seq.len() {
            match seq[i] {
                b'(' => {
                    stack.push_front(i);
                    i += 1;
                }
                b')' => {
                    max_d = std::cmp::max(max_d, stack.len() / 2 + stack.len() % 2);
                    // get half of the depth into B
                    let mut loop_ = false;
                    while stack.len() > max_d && seq[i] == b')' {
                        loop_ = true;
                        let index = stack.pop_front().unwrap();
                        ve[index] = 1;
                        ve[i] = 1;
                        i += 1;
                    }
                    if !loop_ {
                        i += 1;
                        stack.pop_front();
                    }
                }
                _ => unreachable!(),
            }
        }

        ve
    }
}

#[test]
fn case1() {
    assert_eq!(
        Solution::max_depth_after_split("(()())".into()),
        vec![0, 1, 1, 1, 1, 0]
    );
    assert_eq!(
        Solution::max_depth_after_split("()(())()".into()),
        vec![0, 0, 0, 1, 1, 0, 0, 0]
    );
}
