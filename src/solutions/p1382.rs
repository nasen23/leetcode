struct Solution;

use crate::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

type Tree = Option<Rc<RefCell<TreeNode>>>;

impl Solution {
    pub fn balance_bst(root: Tree) -> Tree {
        let ve = bst_to_vec(root);
        rebuild(&ve, 0, ve.len() - 1)
    }
}

fn bst_to_vec(tree: Tree) -> Vec<i32> {
    let mut v = vec![];
    bst_to_vec_impl(&tree, &mut v);
    v
}

fn bst_to_vec_impl(tree: &Tree, vec: &mut Vec<i32>) {
    if let Some(tree) = tree {
        let tree = tree.borrow();
        bst_to_vec_impl(&tree.left, vec);
        vec.push(tree.val);
        bst_to_vec_impl(&tree.right, vec);
    }
}

fn rebuild(vec: &Vec<i32>, fst: usize, lst: usize) -> Tree {
    if fst > lst {
        return None;
    }

    let mid = (fst + lst) / 2;
    let left = match mid.checked_sub(1) {
        Some(res) => rebuild(vec, fst, res),
        None => None,
    };
    let right = rebuild(vec, mid + 1, lst);

    let mut node = TreeNode::new(vec[mid]);
    node.left = left;
    node.right = right;

    Some(Rc::new(RefCell::new(node)))
}
