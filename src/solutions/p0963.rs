use std::collections::HashMap;

impl Solution {
    pub fn min_area_free_rect(points: Vec<Vec<i32>>) -> f64 {
        let mut min_area = std::i32::MAX;
        let mut map: HashMap<(i32, i32, i32), Vec<(i32, i32)>> = HashMap::new();
        for i in 0..points.len() - 1 {
            for j in i + 1..points.len() {
                let xx = points[i][0] + points[j][0];
                let yy = points[i][1] + points[j][1];
                let dx = points[i][0] - points[j][0];
                let dy = points[i][1] - points[j][1];
                let key = (xx, yy, dx * dx + dy * dy);
                let p = (points[i][0], points[i][1]);
                let v = map.entry(key).or_default();
                for q in v.iter() {
                    let area = (dx * (p.1 - q.1) + dy * (q.0 - p.0)).abs();
                    min_area = min_area.min(area);
                }
                v.push(p);
            }
        }
        if min_area == std::i32::MAX {
            0.0
        } else {
            min_area as f64
        }
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vec_vec;

    #[test]
    fn minimum_area_rectangle_ii() {
        assert_eq!(
            2.0,
            Solution::min_area_free_rect(vec_vec![[1, 2], [2, 1], [1, 0], [0, 1]])
        );
        assert_eq!(
            1.0,
            Solution::min_area_free_rect(vec_vec![[0, 1], [2, 1], [1, 1], [1, 0], [2, 0]])
        );
        assert_eq!(
            0.0,
            Solution::min_area_free_rect(vec_vec![[0, 3], [1, 2], [3, 1], [1, 3], [2, 1]])
        );
        assert_eq!(
            2.0,
            Solution::min_area_free_rect(vec_vec![
                [3, 1],
                [1, 1],
                [0, 1],
                [2, 1],
                [3, 3],
                [3, 2],
                [0, 2],
                [2, 3]
            ])
        );
    }
}
