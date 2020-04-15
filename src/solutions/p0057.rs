impl Solution {
    pub fn insert(intervals: Vec<Vec<i32>>, new_interval: Vec<i32>) -> Vec<Vec<i32>> {
        let mut res = Vec::with_capacity(intervals.len());
        let mut start = -1;
        let mut end = -1;
        for interval in intervals {
            if start == -1 {
                // search for start
                if interval[0] >= new_interval[0] {
                    start = new_interval[0];
                } else if interval[1] >= new_interval[0] {
                    start = interval[0];
                } else {
                    res.push(interval);
                    continue;
                }
            }
            if end == -1 {
                // search for end
                if interval[0] > new_interval[1] {
                    end = new_interval[1];
                    res.push(vec![start, end]);
                    res.push(interval);
                } else if interval[1] >= new_interval[1] {
                    end = interval[1];
                    res.push(vec![start, end]);
                }
            } else {
                res.push(interval);
            }
        }
        if start == -1 {
            res.push(new_interval);
        } else if end == -1 {
            res.push(vec![start, new_interval[1]]);
        }

        res
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::vec_vec;

    #[test]
    fn insert_interval() {
        assert_eq!(
            Solution::insert(vec_vec![[1, 3], [6, 9]], vec![2, 5]),
            vec_vec![[1, 5], [6, 9]]
        );
        assert_eq!(
            Solution::insert(
                vec_vec![[1, 2], [3, 5], [6, 7], [8, 10], [12, 16]],
                vec![4, 8]
            ),
            vec_vec![[1, 2], [3, 10], [12, 16]]
        );
        assert_eq!(Solution::insert(vec_vec![], vec![4, 8]), vec_vec![[4, 8]]);
    }
}
