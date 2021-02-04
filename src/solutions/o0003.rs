impl Solution {
    pub fn find_repeat_number(nums: Vec<i32>) -> i32 {
        let mut flag = vec![false; nums.len()];
        for num in nums {
            if flag[num as usize] {
                return num;
            }
            flag[num as usize] = true;
        }
        -1
    }
}

struct Solution;
