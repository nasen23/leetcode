impl Solution {
    pub fn champagne_tower(poured: i32, query_row: i32, query_glass: i32) -> f64 {
        let mut spill = vec![poured as f64];
        for _ in 0..query_row {
            spill = spill
                .into_iter()
                .map(|x| {
                    let x = (x - 1.0).max(0.0) / 2.0;
                    vec![x, x]
                })
                .fold(vec![0.0], |mut x, y| {
                    *x.last_mut().unwrap() += y[0];
                    x.push(y[1]);
                    x
                });
        }
        spill[query_glass as usize].min(1.0)
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn champagne_tower() {
        assert_eq!(0.5, Solution::champagne_tower(2, 1, 1));
        assert_eq!(1.0, Solution::champagne_tower(100000009, 33, 17));
    }
}
