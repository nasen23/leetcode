use crate::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

struct Solution;

type Tree = Option<Rc<RefCell<TreeNode>>>;

impl Solution {
    pub fn prune_tree(root: Tree) -> Tree {
        let root = remove_subtree(root);
        root
    }
}

fn remove_subtree(tree: Tree) -> Tree {
    match tree {
        Some(inner) => {
            let mut node = inner.borrow_mut();
            node.left = remove_subtree(node.left.take());
            node.right = remove_subtree(node.right.take());

            if node.left.is_none() && node.right.is_none() && node.val == 0 {
                None
            } else {
                drop(node);
                Some(inner)
            }
        }
        None => tree,
    }
}
