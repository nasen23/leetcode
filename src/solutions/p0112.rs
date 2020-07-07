use crate::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn has_path_sum(root: Option<Rc<RefCell<TreeNode>>>, sum: i32) -> bool {
        if root.is_none() {
            return false;
        }
        path_sum_ref(&root, sum)
    }
}

fn path_sum_ref(node: &Option<Rc<RefCell<TreeNode>>>, sum: i32) -> bool {
    match node {
        Some(node) => {
            let node = node.borrow();
            if node.left.is_none() && node.right.is_none() {
                node.val == sum
            } else {
                path_sum_ref(&node.left, sum - node.val)
                    || path_sum_ref(&node.right, sum - node.val)
            }
        }
        None => false,
    }
}

struct Solution;
