use std::cell::RefCell;
use std::rc::Rc;
// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}
struct Solution;

impl Solution {
    // O(n) space
    pub fn recover_tree(root: &mut Option<Rc<RefCell<TreeNode>>>) {
        let mut list_nodes = Vec::new();
        dfs_get_nodes(&root, &mut list_nodes);
        list_nodes.sort_unstable();
        dfs_inorder(root, &list_nodes, &mut 0)
    }

    // O(1) space
    pub fn optimize_recover_tree(root: &mut Option<Rc<RefCell<TreeNode>>>) {
        let (mut first_elem, mut second_elem) = (None, None);
        optimize_dfs_inorder(root, &None, &mut first_elem, &mut second_elem);

        let second = second_elem.as_ref().unwrap().borrow().val;
        let first = first_elem.as_ref().unwrap().borrow().val;
        first_elem.as_mut().unwrap().borrow_mut().val = second;
        second_elem.as_mut().unwrap().borrow_mut().val = first;
    }
}

fn optimize_dfs_inorder(
    root: &Option<Rc<RefCell<TreeNode>>>,
    prev_elem: &Option<Rc<RefCell<TreeNode>>>,
    first_elem: &mut Option<Rc<RefCell<TreeNode>>>,
    second_elem: &mut Option<Rc<RefCell<TreeNode>>>,
) {
    if root.is_none() {
        return;
    }

    if let Some(node) = &root {
        let node = node.borrow();
        optimize_dfs_inorder(&node.left, root, first_elem, second_elem);
        if let Some(prev_node) = prev_elem {
            let prev_node = prev_node.borrow();
            if node.val < prev_node.val {
                *first_elem = prev_elem.clone();
            }
            if !first_elem.is_none() && node.val < prev_node.val {
                *second_elem = Some(root.as_ref().unwrap().clone())
            }
        }
        optimize_dfs_inorder(&node.right, root, first_elem, second_elem);
    }
}

fn dfs_inorder(root: &mut Option<Rc<RefCell<TreeNode>>>, nodes: &Vec<i32>, idx: &mut usize) {
    if root.is_none() {
        return;
    }

    let mut node = root.as_mut().unwrap().borrow_mut();
    dfs_inorder(&mut node.left, nodes, idx);

    (*node).val = nodes[*idx];
    *idx += 1;

    dfs_inorder(&mut node.right, nodes, idx);
}

fn dfs_get_nodes(root: &Option<Rc<RefCell<TreeNode>>>, list_nodes: &mut Vec<i32>) {
    if root.is_none() {
        return;
    }

    let node = root.as_ref().unwrap().borrow();
    list_nodes.push(node.val);
    dfs_get_nodes(&node.left, list_nodes);
    dfs_get_nodes(&node.right, list_nodes);
}
