impl Solution {
    pub fn intersection(
        start1: Vec<i32>,
        end1: Vec<i32>,
        start2: Vec<i32>,
        end2: Vec<i32>,
    ) -> Vec<f64> {
        let (x1, y1, x2, y2) = (start1[0], start1[1], end1[0], end1[1]);
        let (x3, y3, x4, y4) = (start2[0], start2[1], end2[0], end2[1]);
        let (a11, a12, b1) = (y2 - y1, x1 - x2, x1 * y2 - x2 * y1);
        let (a21, a22, b2) = (y4 - y3, x3 - x4, x3 * y4 - x4 * y3);
        let (d, d1, d2) = (
            a11 * a22 - a12 * a21,
            b1 * a22 - a12 * b2,
            a11 * b2 - b1 * a21,
        );
        if d == 0 && d1 == 0 {
            if x3.min(x4) > x1.max(x2) || x1.min(x2) > x3.max(x4) {
                return vec![];
            }
            if y3.min(y4) > y1.max(y2) || y1.min(y2) > y3.max(y4) {
                return vec![];
            }
            let mut mat = vec![(x1, y1), (x2, y2), (x3, y3), (x4, y4)];
            mat.sort();
            return vec![mat[1].0 as f64, mat[1].1 as f64];
        }
        if d != 0 {
            let (x0, y0) = (d1 as f64 / d as f64, d2 as f64 / d as f64);
            let inline = |a: f64, b: f64, c: f64| {
                if (a - b).abs() < 1e-6 || (a - c).abs() < 1e-6 {
                    true
                } else {
                    (a > b && a < c) || (a < b && a > c)
                }
            };
            let (x1, x2, x3, x4) = (x1 as f64, x2 as f64, x3 as f64, x4 as f64);
            let (y1, y2, y3, y4) = (y1 as f64, y2 as f64, y3 as f64, y4 as f64);
            if inline(x0, x1, x2) && inline(x0, x3, x4) && inline(y0, y1, y2) && inline(y0, y3, y4)
            {
                return vec![x0, y0];
            }
        }
        vec![]
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn intersection_lcci() {
        assert_eq!(
            Solution::intersection(vec![0, 0], vec![1, 0], vec![1, 1], vec![0, -1]),
            vec![0.5, 0.0]
        );
        assert_eq!(
            Solution::intersection(vec![0, 0], vec![3, 3], vec![1, 1], vec![0, -1]),
            vec![1.0, 1.0]
        );
        assert_eq!(
            Solution::intersection(vec![0, 0], vec![1, 1], vec![1, 0], vec![2, 1]),
            vec![]
        );
        assert_eq!(
            Solution::intersection(vec![0, 3], vec![0, 6], vec![0, 1], vec![0, 5]),
            vec![0.0, 3.0]
        )
    }
}
