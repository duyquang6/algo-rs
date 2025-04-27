// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}
//
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

use std::cell::RefCell;
use std::rc::Rc;
struct Solution;

impl Solution {
    pub fn lowest_common_ancestor(
        root: Option<Rc<RefCell<TreeNode>>>,
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        // let (_, ans) = recur(&root, p.unwrap().borrow().val, q.unwrap().borrow().val);
        // ans

        if let Some(root_rc) = root {
            let node = root_rc.borrow();
            let p_val = p.clone().unwrap().borrow().val;
            let q_val = q.clone().unwrap().borrow().val;

            if node.val == p_val || node.val == q_val {
                return Some(root_rc.clone());
            }

            let left = Solution::lowest_common_ancestor(node.left.clone(), p.clone(), q.clone());
            let right = Solution::lowest_common_ancestor(node.right.clone(), p, q);

            if left.is_some() && right.is_some() {
                return Some(root_rc.clone());
            }

            if right.is_none() {
                return left;
            }
            right
        } else {
            None
        }
    }
}

fn recur(
    root: &Option<Rc<RefCell<TreeNode>>>,
    p_val: i32,
    q_val: i32,
) -> (bool, Option<Rc<RefCell<TreeNode>>>) {
    // dfs to find p node
    if let Some(node_rc) = root {
        let node = node_rc.borrow();
        if node.val == p_val || node.val == q_val {
            // found p or q

            // let say we found p, dive into p subtree to find q
            if dfs(&node.left, p_val + q_val - node.val)
                || dfs(&node.right, p_val + q_val - node.val)
            {
                return (true, Some(node_rc.clone()));
            }
            // go upper one level
            return (true, None);
        } else {
            let (found_left, lca_node_left) = recur(&node.left, p_val, q_val);
            let (found_right, lca_node_right) = recur(&node.right, p_val, q_val);

            if lca_node_left.is_some() {
                return (found_left, lca_node_left);
            }
            if lca_node_right.is_some() {
                return (found_right, lca_node_right);
            }
            if found_left && found_right {
                return (true, Some(node_rc.clone()));
            }
            if found_left {
                return (found_left, None);
            }
            if found_right {
                return (found_right, None);
            }
        }
    }
    (false, None)
}

fn dfs(node: &Option<Rc<RefCell<TreeNode>>>, q_val: i32) -> bool {
    if let Some(node) = node {
        let node = node.borrow();

        if node.val == q_val {
            return true;
        }

        return dfs(&node.left, q_val) || dfs(&node.right, q_val);
    }
    false
}
