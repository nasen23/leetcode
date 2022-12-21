use std::collections::HashMap;

fn calc_split(
    map: &mut HashMap<(usize, usize), i32>,
    arr: &[i32],
    l: usize,
    r: usize,
    sp: usize,
) -> i32 {
    mct_from_leaf_values_impl(map, arr, l, sp)
        + mct_from_leaf_values_impl(map, arr, sp, r)
        + arr[l..sp].iter().max().unwrap() * arr[sp..r].iter().max().unwrap()
}

// returns smallest possible sum of non-leaf in this subtree
fn mct_from_leaf_values_impl(
    map: &mut HashMap<(usize, usize), i32>,
    arr: &[i32],
    l: usize,
    r: usize,
) -> i32 {
    if let Some(&v) = map.get(&(l, r)) {
        return v;
    }
    let v = match r - l {
        1 => 0,
        2 => arr[l] * arr[l + 1],
        _ => (l + 1..r)
            .map(|i| calc_split(map, arr, l, r, i))
            .min()
            .unwrap(),
    };
    map.insert((l, r), v);
    v
    // if number of nodes are even, there is only one way to divide the nodes
}

impl Solution {
    pub fn mct_from_leaf_values(arr: Vec<i32>) -> i32 {
        mct_from_leaf_values_impl(&mut HashMap::new(), &arr, 0, arr.len())
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn min_cost_tree_from_leaf_values() {
        assert_eq!(Solution::mct_from_leaf_values(vec![6, 2, 4]), 32);
        assert_eq!(Solution::mct_from_leaf_values(vec![4, 11]), 44);
        assert_eq!(Solution::mct_from_leaf_values(vec![15, 13, 5, 3, 15]), 500);
    }
}
