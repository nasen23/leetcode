impl Solution {
    pub fn video_stitching(mut clips: Vec<Vec<i32>>, mut t: i32) -> i32 {
        clips.sort_by(|a, b| b[1].cmp(&a[1]));
        let mut cur = t;
        let mut i = 0;
        let mut res = 0;
        while t > 0 {
            if i < clips.len() && clips[i][1] >= t {
                cur = cur.min(clips[i][0]);
                i += 1;
            } else {
                if cur < t {
                    res += 1;
                    t = cur;
                } else {
                    return -1;
                }
            }
        }
        res
    }
}

struct Solution;
