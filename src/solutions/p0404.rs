use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn sum_of_left_leaves(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        match root {
            Some(node) => {
                let node = Rc::try_unwrap(node).unwrap().into_inner();
                if let Some(l) = &node.left {
                    let l = l.borrow();
                    if l.left.is_none() && l.right.is_none() {
                        return l.val + Solution::sum_of_left_leaves(node.right);
                    }
                }
                Solution::sum_of_left_leaves(node.left) + Solution::sum_of_left_leaves(node.right)
            }
            None => 0,
        }
    }
}

use crate::TreeNode;
struct Solution;
