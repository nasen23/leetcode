use std::collections::BinaryHeap;

impl Solution {
    pub fn k_weakest_rows(mat: Vec<Vec<i32>>, k: i32) -> Vec<i32> {
        let k = k as usize;
        let heap = mat
            .into_iter()
            .enumerate()
            .fold(BinaryHeap::new(), |mut heap, (i, row)| {
                let n = row.into_iter().take_while(|&x| x == 1).count();
                heap.push((n, i));
                if heap.len() > k {
                    heap.pop();
                }
                heap
            });
        heap.into_sorted_vec()
            .into_iter()
            .map(|(_, i)| i as i32)
            .collect()
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vec_vec;

    #[test]
    fn k_weakest_rows_in_matrix() {
        assert_eq!(
            vec![2, 0, 3],
            Solution::k_weakest_rows(
                vec_vec![
                    [1, 1, 0, 0, 0],
                    [1, 1, 1, 1, 0],
                    [1, 0, 0, 0, 0],
                    [1, 1, 0, 0, 0],
                    [1, 1, 1, 1, 1]
                ],
                3
            )
        );
    }
}
