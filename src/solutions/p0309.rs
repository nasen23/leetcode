impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        if prices.is_empty() {
            return 0;
        }
        let mut sell = vec![0; prices.len()];
        let mut buy = vec![0; prices.len()];
        buy[0] = -prices[0];
        let mut cool = vec![0; prices.len()];
        for i in 1..prices.len() {
            sell[i] = buy[i - 1] + prices[i];
            buy[i] = (cool[i - 1] - prices[i]).max(buy[i - 1]);
            cool[i] = sell[i - 1].max(cool[i - 1]);
        }
        sell[prices.len() - 1].max(cool[prices.len() - 1])
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn buy_sell_with_cooldown() {
        assert_eq!(Solution::max_profit(vec![1, 2, 3, 0, 2]), 3);
    }
}
