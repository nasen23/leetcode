impl Solution {
    pub fn can_finish(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> bool {
        let n = num_courses as usize;
        let pre = prerequisites
            .into_iter()
            .fold(vec![vec![]; n], |mut v, pair| {
                v[pair[0] as usize].push(pair[1] as usize);
                v
            });
        let mut vis = vec![false; n];
        let mut tmp = vec![false; n];

        fn dfs(i: usize, vis: &mut Vec<bool>, tmp: &mut Vec<bool>, pre: &Vec<Vec<usize>>) -> bool {
            vis[i] = true;
            tmp[i] = true;
            for &k in &pre[i] {
                if tmp[k] || (!vis[k] && !dfs(k, vis, tmp, pre)) {
                    return false;
                }
            }
            tmp[i] = false;
            true
        }

        for i in 0..n {
            if !vis[i] {
                if !dfs(i, &mut vis, &mut tmp, &pre) {
                    return false;
                }
            }
        }
        true
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vec_vec;

    #[test]
    fn course_schedule() {
        assert_eq!(Solution::can_finish(2, vec_vec![[0, 1]]), true);
        assert_eq!(Solution::can_finish(2, vec_vec![[1, 0], [0, 1]]), false);
        assert_eq!(
            Solution::can_finish(3, vec_vec![[1, 0], [0, 2], [2, 1]]),
            false
        );
    }
}
