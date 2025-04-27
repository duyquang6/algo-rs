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
use std::cell::RefCell;
use std::rc::Rc;
use std::str::from_utf8;
struct Codec {}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Codec {
    fn new() -> Self {
        Codec {}
    }

    fn serialize(&self, root: Option<Rc<RefCell<TreeNode>>>) -> String {
        let mut deserialize_data = String::new();

        if root.is_some() {
            write_node(&mut deserialize_data, &root);
        } else {
            return deserialize_data;
        }

        recur_serialize(0, &mut deserialize_data, root);

        // pop last ','
        deserialize_data.pop();
        while deserialize_data.as_bytes()[deserialize_data.len() - 1] == b'N' {
            deserialize_data.pop();
            deserialize_data.pop();
        }

        deserialize_data
    }

    fn deserialize(&self, data: String) -> Option<Rc<RefCell<TreeNode>>> {
        if data.len() == 0 {
            return None;
        }
        let data_arr = data
            .split(',')
            .map(|x| {
                if x != "N" {
                    Some(x.parse::<i32>().unwrap())
                } else {
                    None
                }
            })
            .collect::<Vec<Option<i32>>>();
        let mut root_node = TreeNode::new(data_arr[0].unwrap());
        let root_node = Rc::new(RefCell::new(root_node));

        recur_deserialize(&data_arr, &mut 1, root_node.clone());

        Some(root_node)
    }
}

fn write_node(data: &mut String, node: &Option<Rc<RefCell<TreeNode>>>) {
    if let Some(node) = node {
        let node = node.borrow();
        data.push_str(&node.val.to_string());
    } else {
        data.push('N');
    }
    data.push(',');
}

fn recur_serialize(idx: usize, data: &mut String, root: Option<Rc<RefCell<TreeNode>>>) {
    if let Some(root_node) = root {
        let root_node = root_node.borrow();

        // write
        write_node(data, &root_node.left);
        write_node(data, &root_node.right);

        recur_serialize(idx * 2 + 1, data, root_node.left.clone());
        recur_serialize(idx * 2 + 2, data, root_node.right.clone());
    }
}

fn recur_deserialize(data: &Vec<Option<i32>>, idx: &mut usize, node: Rc<RefCell<TreeNode>>) {
    if *idx >= data.len() {
        // done deser
        return;
    }

    let mut node = node.borrow_mut();
    node.left = if *idx >= data.len() || data[*idx].is_none() {
        None
    } else {
        Some(Rc::new(RefCell::new(TreeNode::new(data[*idx].unwrap()))))
    };
    *idx += 1;

    node.right = if *idx >= data.len() || data[*idx].is_none() {
        None
    } else {
        Some(Rc::new(RefCell::new(TreeNode::new(data[*idx].unwrap()))))
    };
    *idx += 1;

    if node.left.is_some() {
        recur_deserialize(data, idx, node.left.clone().unwrap())
    }
    if node.right.is_some() {
        recur_deserialize(data, idx, node.right.clone().unwrap())
    }
}
