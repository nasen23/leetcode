use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn is_balanced(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        is_balance_impl(root).1
    }
}

fn is_balance_impl(tree: Option<Rc<RefCell<TreeNode>>>) -> (i32, bool) {
    match tree {
        Some(node) => {
            let node = Rc::try_unwrap(node).unwrap().into_inner();
            let (d1, b1) = is_balance_impl(node.left);
            let (d2, b2) = is_balance_impl(node.right);
            if b1 && b2 && (d1 - d2).abs() <= 1 {
                (d1.max(d2) + 1, true)
            } else {
                (0, false)
            }
        }
        None => (0, true),
    }
}

use crate::TreeNode;
struct Solution;
