use std::cell::RefCell;
use std::rc::Rc;

fn left_mistake(root: &Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
    if root.is_none() {
        return None;
    }
    let node = root.as_ref().unwrap().borrow();
    let res = left_mistake(&node.left);
    if res.is_some() {
        return res;
    }
    let prev = r_most(&node.left);
    if let Some(p) = &prev {
        if p.borrow().val > node.val {
            return prev;
        }
    }
    let next = l_most(&node.right);
    if let Some(n) = next {
        if n.borrow().val < node.val {
            return root.clone();
        }
    }
    left_mistake(&node.right)
}

fn right_mistake(root: &Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
    if root.is_none() {
        return None;
    }
    let node = root.as_ref().unwrap().borrow();
    let res = right_mistake(&node.right);
    if res.is_some() {
        return res;
    }
    let prev = l_most(&node.right);
    if let Some(p) = &prev {
        if p.borrow().val < node.val {
            return prev;
        }
    }
    let next = r_most(&node.left);
    if let Some(n) = next {
        if n.borrow().val > node.val {
            return root.clone();
        }
    }
    right_mistake(&node.left)
}

fn l_most(root: &Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
    match root {
        None => None,
        Some(node) => match node.borrow().left.as_ref() {
            None => root.clone(),
            Some(left) => l_most(&Some(left.clone())),
        },
    }
}

fn r_most(root: &Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
    match root {
        None => None,
        Some(node) => match node.borrow().right.as_ref() {
            None => root.clone(),
            Some(right) => r_most(&Some(right.clone())),
        },
    }
}

impl Solution {
    pub fn recover_tree(root: &mut Option<Rc<RefCell<TreeNode>>>) {
        let tl = left_mistake(root).unwrap();
        let tr = right_mistake(root).unwrap();
        let tmp = tl.borrow().val;
        tl.borrow_mut().val = tr.borrow_mut().val;
        tr.borrow_mut().val = tmp;
    }
}

use crate::TreeNode;
struct Solution;
