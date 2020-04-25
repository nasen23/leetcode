impl Solution {
    pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut nums = nums;
        let mut res = vec![];

        fn recursive_select(start: usize, nums: &mut Vec<i32>, res: &mut Vec<Vec<i32>>) {
            if start == nums.len() {
                res.push(nums.clone());
            }

            for i in start..nums.len() {
                nums.swap(i, start);
                recursive_select(start + 1, nums, res);
                nums.swap(i, start);
            }
        }

        recursive_select(0, &mut nums, &mut res);
        res
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::vec_vec;

    #[test]
    fn permutations() {
        let res = vec_vec![
            [1, 2, 3],
            [1, 3, 2],
            [2, 1, 3],
            [2, 3, 1],
            [3, 1, 2],
            [3, 2, 1]
        ];
        let mut input = Solution::permute(vec![1, 2, 3]);
        input.sort_unstable();
        assert_eq!(input, res);
    }
}
