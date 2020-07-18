impl Solution {
    pub fn combination_sum2(mut nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        nums.sort();
        let mut res = vec![];
        combination_sum2_impl(&nums, &mut vec![], &mut res, target);
        res.into_iter().collect()
    }
}

fn combination_sum2_impl(nums: &[i32], tmp: &mut Vec<i32>, res: &mut Vec<Vec<i32>>, t: i32) {
    if t == 0 {
        res.push(tmp.clone());
        return;
    }
    if nums.len() == 0 {
        return;
    }
    if *nums.last().unwrap() * (nums.len() as i32) < t {
        return;
    }
    for i in (0..nums.len())
        .take_while(|&i| nums[i] <= t)
        .filter(|&i| i == 0 || nums[i] != nums[i - 1])
    {
        tmp.push(nums[i]);
        combination_sum2_impl(&nums[i + 1..], tmp, res, t - nums[i]);
        tmp.pop();
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vec_vec;

    #[test]
    fn combination_sum2() {
        // assert_eq!(
        //     Solution::combination_sum2(vec![10, 1, 2, 7, 6, 1, 5], 8),
        //     vec_vec![[1, 7], [1, 2, 5], [2, 6], [1, 1, 6]]
        // );
        assert_eq!(
            Solution::combination_sum2(
                vec![
                    14, 6, 25, 9, 30, 20, 33, 34, 28, 30, 16, 12, 31, 9, 9, 12, 34, 16, 25, 32, 8,
                    7, 30, 12, 33, 20, 21, 29, 24, 17, 27, 34, 11, 17, 30, 6, 32, 21, 27, 17, 16,
                    8, 24, 12, 12, 28, 11, 33, 10, 32, 22, 13, 34, 18, 12
                ],
                27
            ),
            vec_vec![[1, 7], [1, 2, 5], [2, 6], [1, 1, 6]]
        );
    }
}
