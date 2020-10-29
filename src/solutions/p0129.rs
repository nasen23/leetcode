use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn sum_numbers(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        vec_nums(&root)
            .into_iter()
            .map(|v| {
                v.into_iter()
                    .enumerate()
                    .map(|(i, x)| x * 10i32.pow(i as u32))
                    .sum::<i32>()
            })
            .sum()
    }
}

fn vec_nums(root: &Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
    if let Some(node) = root {
        let node = node.borrow();
        let left = vec_nums(&node.left);
        let right = vec_nums(&node.right);
        if left.is_empty() && right.is_empty() {
            return vec![vec![node.val]];
        }
        left.into_iter()
            .chain(right.into_iter())
            .map(|mut v| {
                v.push(node.val);
                v
            })
            .collect()
    } else {
        vec![]
    }
}

use crate::TreeNode;
struct Solution;
