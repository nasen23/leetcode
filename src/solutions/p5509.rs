impl Solution {
    pub fn min_cost(s: String, cost: Vec<i32>) -> i32 {
        let by = s.as_bytes();
        let mut i = 0;
        let mut res = 0;
        while i < by.len() - 1 {
            if by[i + 1] == by[i] {
                let mut s = cost[i] + cost[i + 1];
                let mut m = cost[i].max(cost[i + 1]);
                let mut j = i + 2;
                while j < by.len() && by[j] == by[i] {
                    s += cost[j];
                    m = m.max(cost[j]);
                    j += 1;
                }
                res += s - m;
                i = j - 1;
            }
            i += 1;
        }
        res
    }
}

struct Solution;
