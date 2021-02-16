const MOD: i64 = 1000000007;

impl Solution {
    pub fn count_homogenous(s: String) -> i32 {
        let s = s.as_bytes();
        let mut res = 0;
        let mut i = 0;
        while i < s.len() {
            let mut tmp: i64 = 1;
            let mut j = i + 1;
            while j < s.len() {
                if s[j] == s[j - 1] {
                    tmp += 1;
                    j += 1;
                } else {
                    break;
                }
            }
            res += (tmp * (tmp + 1) / 2 % MOD) as i32;
            i = j;
        }
        res
    }
}

struct Solution;
