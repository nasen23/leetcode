impl Solution {
    pub fn interval_intersection(a: Vec<Vec<i32>>, b: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let (mut i, mut j) = (0, 0);
        let mut res = vec![];
        while i < a.len() && j < b.len() {
            if a[i][1] < b[j][0] {
                i += 1;
            } else if b[j][1] < a[i][0] {
                j += 1;
            } else if a[i][1] < b[j][1] {
                res.push(vec![a[i][0].max(b[j][0]), a[i][1]]);
                i += 1;
            } else {
                res.push(vec![a[i][0].max(b[j][0]), b[j][1]]);
                j += 1;
            }
        }
        res
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vec_vec;

    #[test]
    fn interval_intersection() {
        assert_eq!(
            Solution::interval_intersection(
                vec_vec![[0, 2], [5, 10], [13, 23], [24, 25]],
                vec_vec![[1, 5], [8, 12], [15, 24], [25, 26]]
            ),
            vec_vec![[1, 2], [5, 5], [8, 10], [15, 23], [24, 24], [25, 25]]
        )
    }
}
