impl Solution {
    pub fn array_nesting(nums: Vec<i32>) -> i32 {
        let nums = nums.into_iter().map(|x| x as usize).collect::<Vec<_>>();
        let mut visited = vec![false; nums.len()];
        let mut res = 1;
        for i in 0..nums.len() {
            if visited[i] {
                continue;
            }
            let mut tmp = 0;
            let mut j = i;
            while !visited[j] {
                visited[j] = true;
                tmp += 1;
                j = nums[j];
            }
            res = res.max(tmp);
        }
        res
    }
}

struct Solution;
