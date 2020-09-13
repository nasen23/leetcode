use std::collections::{HashMap, VecDeque};

impl Solution {
    pub fn find_order(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> Vec<i32> {
        let n = num_courses as usize;
        let mut pre = vec![0; n];
        let mut post = HashMap::new();
        for v in &prerequisites {
            pre[v[0] as usize] += 1;
            post.entry(v[1] as usize)
                .or_insert(vec![])
                .push(v[0] as usize);
        }
        let mut res = vec![];
        let mut queue = VecDeque::new();
        for i in 0..n {
            if pre[i] == 0 {
                queue.push_back(i);
            }
        }
        while let Some(i) = queue.pop_front() {
            res.push(i as i32);
            if let Some(v) = post.get(&i) {
                for &j in v {
                    pre[j] -= 1;
                    if pre[j] == 0 {
                        queue.push_back(j);
                    }
                }
            }
        }
        if pre.into_iter().all(|v| v == 0) {
            return res;
        }
        vec![]
    }
}

struct Solution;
