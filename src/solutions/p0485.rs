impl Solution {
    pub fn find_max_consecutive_ones(nums: Vec<i32>) -> i32 {
        let mut res = 0;
        let mut tmp = 0;
        for num in nums {
            if num == 0 {
                tmp = 0;
            } else {
                tmp += 1;
                res = res.max(tmp);
            }
        }
        res
    }
}

struct Solution;
