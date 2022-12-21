impl Solution {
    pub fn max_satisfied(customers: Vec<i32>, grumpy: Vec<i32>, x: i32) -> i32 {
        let x = x as usize;
        let mut sat = customers
            .iter()
            .zip(grumpy.iter())
            .filter(|(_, b)| **b == 0)
            .map(|(a, _)| *a)
            .sum::<i32>();
        sat += customers
            .iter()
            .take(x)
            .zip(grumpy.iter())
            .filter(|(_, b)| **b == 1)
            .map(|(a, _)| *a)
            .sum::<i32>();
        let mut res = sat;
        for i in 0..customers.len() - x {
            if grumpy[i] == 1 {
                sat -= customers[i];
            }
            if grumpy[i + x] == 1 {
                sat += customers[i + x];
            }
            res = res.max(sat);
        }
        res
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn grumpy_bookstore_owner() {
        assert_eq!(
            16,
            Solution::max_satisfied(
                vec![1, 0, 1, 2, 1, 1, 7, 5],
                vec![0, 1, 0, 1, 0, 1, 0, 1],
                3
            )
        );
        assert_eq!(13, Solution::max_satisfied(vec![5, 8], vec![0, 1], 1));
    }
}
