use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn min_camera_cover(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        min_cover_impl(&root).1
    }
}

fn min_cover_impl(root: &Option<Rc<RefCell<TreeNode>>>) -> (i32, i32, i32) {
    match root {
        Some(inner) => {
            let (la, lb, lc) = min_cover_impl(&inner.borrow().left);
            let (ra, rb, rc) = min_cover_impl(&inner.borrow().right);
            let a = lc + rc + 1;
            (a, a.min(la + rb).min(lb + ra), a.min(lb + rb))
        }
        None => (i32::MAX >> 1, 0, 0),
    }
}

use crate::TreeNode;
struct Solution;
