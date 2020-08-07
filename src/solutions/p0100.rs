use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn is_same_tree(
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        match (p, q) {
            (Some(p), Some(q)) => {
                let p = Rc::try_unwrap(p).unwrap().into_inner();
                let q = Rc::try_unwrap(q).unwrap().into_inner();
                p.val == q.val
                    && Solution::is_same_tree(p.left, q.left)
                    && Solution::is_same_tree(p.right, q.right)
            }
            (None, None) => true,
            _ => false,
        }
    }
}

use crate::TreeNode;
struct Solution;
