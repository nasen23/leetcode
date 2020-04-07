struct Solution;

impl Solution {
    pub fn min_time_to_visit_all_points(points: Vec<Vec<i32>>) -> i32 {
        let start = (points[0][0], points[0][1]);
        points
            .into_iter()
            .skip(1)
            .fold((start, 0), |(last, acc), p| {
                (
                    (p[0], p[1]),
                    acc + std::cmp::max((p[0] - last.0).abs(), (p[1] - last.1).abs()),
                )
            })
            .1
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::vec_vec;

    #[test]
    fn test() {
        assert_eq!(
            Solution::min_time_to_visit_all_points(vec_vec![[1, 1], [3, 4], [-1, 0]]),
            7
        );
        assert_eq!(
            Solution::min_time_to_visit_all_points(vec_vec![[3, 2], [-2, 2]]),
            5
        );
    }
}
