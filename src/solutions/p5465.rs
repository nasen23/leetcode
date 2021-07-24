impl Solution {
    pub fn count_sub_trees(n: i32, edges: Vec<Vec<i32>>, labels: String) -> Vec<i32> {
        let l = labels.as_bytes();
        let mut v = vec![vec![]; n as usize];
        for e in edges {
            let (x, y) = (e[0], e[1]);
            v[x as usize].push(y);
            v[y as usize].push(x);
        }
        let mut s = vec![vec![0; 26]; n as usize];
        let mut res = vec![0; n as usize];
        fn dfs(
            x: i32,
            pre: i32,
            v: &Vec<Vec<i32>>,
            l: &[u8],
            s: &mut Vec<Vec<i32>>,
            res: &mut Vec<i32>,
        ) {
            s[x as usize][(l[x as usize] - b'a') as usize] += 1;
            for &y in &v[x as usize] {
                if y == pre {
                    continue;
                }
                dfs(y, x, v, l, s, res);
                for i in 0..26 {
                    s[x as usize][i] += s[y as usize][i];
                }
            }
            res[x as usize] = s[x as usize][(l[x as usize] - b'a') as usize];
        }
        dfs(0, -1, &v, l, &mut s, &mut res);
        res
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vec_vec;

    #[test]
    fn count_sub_trees() {
        assert_eq!(
            Solution::count_sub_trees(
                7,
                vec_vec![[0, 1], [0, 2], [1, 4], [1, 5], [2, 3], [2, 6]],
                "abaedcd".into()
            ),
            vec![2, 1, 1, 1, 1, 1, 1]
        );
        assert_eq!(
            Solution::count_sub_trees(
                6,
                vec_vec![[0, 1], [0, 2], [1, 3], [3, 4], [4, 5]],
                "cbabaa".into()
            ),
            vec![1, 2, 1, 1, 2, 1]
        );
        assert_eq!(
            Solution::count_sub_trees(4, vec_vec![[0, 1], [1, 2], [0, 3]], "bbbb".into()),
            vec![4, 2, 1, 1]
        );
        assert_eq!(
            Solution::count_sub_trees(
                7,
                vec_vec![[0, 1], [1, 2], [2, 3], [3, 4], [4, 5], [5, 6]],
                "aaabaaa".into()
            ),
            vec![6, 5, 4, 1, 3, 2, 1]
        );
    }
}
