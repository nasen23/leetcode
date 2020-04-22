use crate::TreeNode;

use std::cell::RefCell;
use std::rc::Rc;

type Tree = Option<Rc<RefCell<TreeNode>>>;

impl Solution {
    pub fn insert_into_bst(root: Tree, val: i32) -> Tree {
        if root.is_none() {
            return Some(Rc::new(RefCell::new(TreeNode::new(val))));
        }

        let mut node = root.as_ref().unwrap().clone();
        loop {
            let mut borrow = node.borrow_mut();
            let new_node;
            if borrow.val > val {
                // check left
                match &borrow.left {
                    Some(left) => new_node = left.clone(),
                    None => {
                        // insert
                        borrow.left = Some(Rc::new(RefCell::new(TreeNode::new(val))));
                        break;
                    }
                }
            } else {
                match &borrow.right {
                    Some(right) => new_node = right.clone(),
                    None => {
                        // insert
                        borrow.right = Some(Rc::new(RefCell::new(TreeNode::new(val))));
                        break;
                    }
                }
            }
            drop(borrow);
            node = new_node;
        }

        root
    }
}

struct Solution;
