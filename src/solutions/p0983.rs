impl Solution {
    pub fn mincost_tickets(days: Vec<i32>, costs: Vec<i32>) -> i32 {
        // I am so weak
        let last_day = *days.last().unwrap() as usize;
        let mut dp = vec![0; last_day + 1];
        for day in days {
            dp[day as usize] = -1;
        }

        for i in 1..dp.len() {
            if dp[i] == 0 {
                dp[i] = dp[i - 1];
            } else {
                let c0 = dp[i - 1] + costs[0];

                let c1 = if i >= 7 {
                    dp[i - 7] + costs[1]
                } else {
                    costs[1]
                };
                let c2 = if i >= 30 {
                    dp[i - 30] + costs[2]
                } else {
                    costs[2]
                };

                dp[i] = std::cmp::min(c0, c1);
                dp[i] = std::cmp::min(dp[i], c2);
            }
        }

        dp[last_day]
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn minimum_cost_for_tickets() {
        assert_eq!(
            Solution::mincost_tickets(vec![1, 4, 6, 7, 8, 20], vec![2, 7, 15]),
            11
        );
    }
}
