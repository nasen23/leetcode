impl Solution {
    pub fn matrix_reshape(nums: Vec<Vec<i32>>, r: i32, c: i32) -> Vec<Vec<i32>> {
        let (r, c) = (r as usize, c as usize);
        if nums.len() * nums[0].len() == r * c {
            nums.concat().chunks(c).map(|v| v.to_vec()).collect()
        } else {
            nums
        }
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vec_vec;

    #[test]
    fn reshape_the_matrix() {
        assert_eq!(
            vec_vec![[1, 2, 3, 4]],
            Solution::matrix_reshape(vec_vec![[1, 2], [3, 4]], 1, 4)
        );
    }
}
