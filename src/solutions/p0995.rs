impl Solution {
    pub fn min_k_bit_flips(mut a: Vec<i32>, k: i32) -> i32 {
        let k = k as usize;
        let mut res = 0;
        let mut rev = 0;
        for i in 0..a.len() {
            if i >= k && a[i - k] > 1 {
                rev ^= 1;
            }
            if a[i] == rev {
                if i + k > a.len() {
                    return -1;
                }
                rev ^= 1;
                res += 1;
                a[i] += 2;
            }
        }
        res
    }
}

struct Solution;
