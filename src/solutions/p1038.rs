use std::cell::RefCell;
use std::rc::Rc;

fn bst_to_gst_impl(node: &mut Option<Rc<RefCell<TreeNode>>>, pre: &mut i32) {
    if let Some(node) = node {
        let mut node = node.borrow_mut();
        bst_to_gst_impl(&mut node.right, pre);
        *pre += node.val;
        node.val = *pre;
        bst_to_gst_impl(&mut node.left, pre);
    }
}

impl Solution {
    pub fn bst_to_gst(mut root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        bst_to_gst_impl(&mut root, &mut 0);
        root
    }
}

use crate::TreeNode;
struct Solution;
