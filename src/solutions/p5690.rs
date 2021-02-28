fn union(mut a: Vec<bool>, b: Vec<bool>) -> Vec<bool> {
    for i in 0..a.len().min(b.len()) {
        if b[i] {
            a[i] = true;
        }
    }
    a
}

fn shift(mut a: Vec<bool>, k: usize) -> Vec<bool> {
    for i in (k..a.len()).rev() {
        a[i] = a[i - k];
    }
    for i in 0..k.min(a.len()) {
        a[i] = false;
    }
    a
}

impl Solution {
    pub fn closest_cost(base_costs: Vec<i32>, topping_costs: Vec<i32>, target: i32) -> i32 {
        let mut costs = vec![false; 20005];
        for cost in base_costs {
            costs[cost as usize] = true;
        }
        for cost in topping_costs {
            costs = union(
                costs.clone(),
                union(
                    shift(costs.clone(), cost as usize),
                    shift(costs, (cost * 2) as usize),
                ),
            );
        }
        let mut res = -20000;
        for i in 0..20005 {
            if costs[i] && (i as i32 - target).abs() < (res - target).abs() {
                res = i as i32;
            }
        }
        res
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn closest_dessert_cost() {
        assert_eq!(10, Solution::closest_cost(vec![1, 7], vec![3, 4], 10));
        assert_eq!(17, Solution::closest_cost(vec![2, 3], vec![4, 5, 100], 18));
    }
}
