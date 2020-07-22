impl Solution {
    pub fn merge(mut intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        if intervals.is_empty() {
            return vec![];
        }
        intervals.sort_unstable_by_key(|v| v[0]);
        let (mut px, mut py) = (intervals[0][0], intervals[0][1]);
        let mut res = vec![];
        for v in intervals.into_iter().skip(1) {
            let (x, y) = (v[0], v[1]);
            if py >= x {
                py = py.max(y);
            } else {
                res.push(vec![px, py]);
                px = x;
                py = y;
            }
        }
        res.push(vec![px, py]);
        res
    }
}

struct Solution;
