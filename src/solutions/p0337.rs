use std::cell::RefCell;
use std::rc::Rc;

// returns (if rob this node max, if not rob this node max)
fn try_rob(node: Option<Rc<RefCell<TreeNode>>>) -> (i32, i32) {
    match node {
        Some(node) => {
            let node = Rc::try_unwrap(node).unwrap().into_inner();
            let l = try_rob(node.left);
            let r = try_rob(node.right);
            (node.val + l.1 + r.1, l.0.max(l.1) + r.0.max(r.1))
        }
        None => (0, 0),
    }
}

impl Solution {
    pub fn rob(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let (a, b) = try_rob(root);
        a.max(b)
    }
}

use crate::TreeNode;
struct Solution;
