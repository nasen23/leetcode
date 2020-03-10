struct Solution;

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        // recording the min value before i
        let mut min = std::i32::MAX;
        let mut profit = 0;

        use std::cmp;
        for &price in &prices {
            min = cmp::min(min, price);
            profit = cmp::max(profit, price - min);
        }

        profit
    }
}
