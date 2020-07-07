impl Solution {
    pub fn array_rank_transform(arr: Vec<i32>) -> Vec<i32> {
        let (mut max, mut min) = (i32::min_value(), i32::max_value());
        for i in 0..arr.len() {
            if arr[i] > max {
                max = arr[i];
            }
            if arr[i] < min {
                min = arr[i];
            }
        }
        let mut bag = vec![0; (max - min) as usize + 1];
        (0..arr.len()).for_each(|i| {
            bag[(arr[i] - min) as usize] = 1;
        });
        (1..bag.len()).for_each(|i| {
            bag[i] += bag[i - 1];
        });
        arr.iter().map(|&x| bag[(x - min) as usize]).collect()
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rank_transform_of_array() {
        assert_eq!(
            Solution::array_rank_transform(vec![40, 10, 20, 30]),
            vec![4, 1, 2, 3]
        );
        assert_eq!(
            Solution::array_rank_transform(vec![100, 100, 100]),
            vec![1, 1, 1]
        );
        assert_eq!(
            Solution::array_rank_transform(vec![100, 100, 100, 200]),
            vec![1, 1, 1, 2]
        );
    }
}
