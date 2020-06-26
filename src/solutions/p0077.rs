impl Solution {
    pub fn combine(n: i32, k: i32) -> Vec<Vec<i32>> {
        // REVIEW: need to evaluate this carefully
        let mut nums = (1..=k).collect::<Vec<_>>();
        nums.push(n + 1);
        let mut res = vec![];
        let mut j = 0;
        while j < k as usize {
            res.push(nums[0..k as usize].to_vec());
            j = 0;
            while j < k as usize && nums[j] + 1 == nums[j + 1] {
                nums[j] = j as i32 + 1;
                j += 1;
            }
            nums[j] += 1;
        }
        res
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vec_vec;

    #[test]
    fn combinations() {
        assert_eq!(
            Solution::combine(4, 2),
            vec_vec![[2, 4], [3, 4], [2, 3], [1, 2], [1, 3], [1, 4]]
        );
    }
}
