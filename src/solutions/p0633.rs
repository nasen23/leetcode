impl Solution {
    pub fn judge_square_sum(c: i32) -> bool {
        let (mut i, mut j) = (0, (c as f64).sqrt().floor() as i32);
        while i <= j {
            let s = i * i + j * j;
            if s == c {
                return true;
            } else if s < c {
                i += 1;
            } else {
                j -= 1;
            }
        }
        false
    }
}

struct Solution;
