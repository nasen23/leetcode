use crate::TreeNode;

use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        Solution::is_valid_bst_impl(&root, &mut std::i64::MIN)
    }

    fn is_valid_bst_impl(node: &Option<Rc<RefCell<TreeNode>>>, last: &mut i64) -> bool {
        match node {
            Some(node) => {
                let node = node.borrow();
                if Solution::is_valid_bst_impl(&node.left, last) {
                    if *last < node.val as i64 {
                        *last = node.val as i64;
                        return Solution::is_valid_bst_impl(&node.right, last);
                    }
                }

                false
            }
            None => true,
        }
    }
}

struct Solution;
