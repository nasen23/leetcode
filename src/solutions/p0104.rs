use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn helper(node: &Option<Rc<RefCell<TreeNode>>>) -> i32 {
            match node {
                Some(node) => {
                    let node = node.borrow();
                    1 + helper(&node.left).max(helper(&node.right))
                }
                None => 0,
            }
        }
        helper(&root)
    }
}

use crate::TreeNode;
struct Solution;
