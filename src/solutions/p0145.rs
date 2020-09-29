use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn postorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut res = vec![];
        let mut st = vec![];
        if let Some(root) = root {
            st.push(root);
        }
        while let Some(node) = st.pop() {
            let mut b = node.borrow_mut();
            res.push(b.val);
            if let Some(x) = b.left.take() {
                st.push(x);
            }
            if let Some(x) = b.right.take() {
                st.push(x);
            }
        }
        res.reverse();
        res
    }
}

use crate::TreeNode;
struct Solution;
