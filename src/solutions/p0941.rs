impl Solution {
    pub fn valid_mountain_array(a: Vec<i32>) -> bool {
        let mut up = true;
        let mut first = true;
        for i in 1..a.len() {
            if up && a[i] > a[i - 1] {
                first = false;
            } else if !up && a[i] < a[i - 1] {
            } else if up && a[i] < a[i - 1] {
                if first {
                    return false;
                }
                up = false;
            } else {
                return false;
            }
        }
        !first && !up
    }
}

struct Solution;
