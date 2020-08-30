impl Solution {
    pub fn contains_pattern(arr: Vec<i32>, m: i32, k: i32) -> bool {
        let m = m as usize;
        let k = k as usize;
        for i in 0..arr.len() - m {
            let mut n = 0;
            while i + (n + 2) * m <= arr.len() {
                if arr[i + n * m..i + (n + 1) * m] == arr[i + (n + 1) * m..i + (n + 2) * m] {
                    n += 1;
                    if n >= k - 1 {
                        break;
                    }
                } else {
                    break;
                }
            }
            if n >= k - 1 {
                return true;
            }
        }
        false
    }
}

struct Solution;
