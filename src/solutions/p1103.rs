impl Solution {
    pub fn distribute_candies(candies: i32, n: i32) -> Vec<i32> {
        let mut c = candies;
        let n = n as usize;
        let mut res = vec![0; n];
        let mut i = 0;
        while c > 0 {
            res[i % n] += (i as i32 + 1).min(c);
            i += 1;
            c -= i as i32;
        }
        res
    }
}

struct Solution;
