use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn flatten(root: &mut Option<Rc<RefCell<TreeNode>>>) {
        match root {
            Some(node) => {
                let left = node_vec(node.borrow_mut().left.take());
                let right = node_vec(node.borrow_mut().right.take());
                let mut prev = None;
                for r in right.into_iter().rev() {
                    r.borrow_mut().right = prev;
                    prev = Some(r);
                }
                for r in left.into_iter().rev() {
                    r.borrow_mut().right = prev;
                    prev = Some(r);
                }
                node.borrow_mut().right = prev;
            }
            None => {}
        }
    }
}

fn node_vec(node: Option<Rc<RefCell<TreeNode>>>) -> Vec<Rc<RefCell<TreeNode>>> {
    match node {
        Some(node) => {
            let mut left = node_vec(node.borrow_mut().left.take());
            let mut right = node_vec(node.borrow_mut().right.take());
            let mut res = vec![node];
            res.append(&mut left);
            res.append(&mut right);
            res
        }
        None => vec![],
    }
}

struct Solution;
use crate::TreeNode;
