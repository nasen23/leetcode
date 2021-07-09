impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        let (mut candidate, mut count) = (0, 0);
        for &num in &nums {
            if num == candidate {
                count += 1;
            } else if count == 0 {
                candidate = num;
            } else {
                count -= 1;
            }
        }

        let siz = nums.iter().filter(|&&x| x == candidate).count();
        if siz * 2 > nums.len() {
            candidate
        } else {
            -1
        }
    }
}

struct Solution;
