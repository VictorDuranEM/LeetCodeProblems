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
      right: None
    }
  }
}

use std::rc::Rc;
use std::cell::RefCell;

pub fn main() {
    let root_tree = Rc::new(RefCell::new(TreeNode::new(1)));
    let root_node: Option<Rc<RefCell<TreeNode>>> = Some(Rc::clone(&root_tree));
    
    let tree_in_first_node_left = Some(Rc::new(RefCell::new(TreeNode::new(2))));
    root_tree.borrow_mut().left = tree_in_first_node_left;
    // println!("{:#?}", root_node);
    println!("{}", diameter_of_binary_tree(root_node));
}

pub fn diameter_of_binary_tree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    if root.is_none() {
        return 0;
    }

    let mut vec: Vec<i32> = Vec::new();
    traverse_all_tree(&root, &mut vec);

    let max = vec.iter().max();

    println!("{:#?}", vec);

    if let Some(x) = max {
        return *x;
    }
    0
}

pub fn traverse_all_tree(node: &Option<Rc<RefCell<TreeNode>>>, vec: &mut Vec<i32>) {
    if let Some(x) = node {
        let left_node = &x.borrow().left;
        let right_node = &x.borrow().right;

        vec.push(get_node_length(node));

        traverse_all_tree(left_node, vec);
        traverse_all_tree(right_node, vec);
    }
}

pub fn get_node_length(node: &Option<Rc<RefCell<TreeNode>>>) -> i32 {
    let root_node = node.as_ref().unwrap();
    
    let first_left_node = &root_node.borrow().left;
    let first_right_node = &root_node.borrow().right;

    let mut left_farthest_position = 0;
    let mut right_farthest_position = 0;
    

    left_farthest_position = get_farthest_level(first_left_node, 0);
    right_farthest_position = get_farthest_level(first_right_node, 0);


    left_farthest_position + right_farthest_position
}

pub fn get_farthest_level(node: &Option<Rc<RefCell<TreeNode>>>, beginning_position: i32) -> i32 {
    let mut position = beginning_position;

    if let Some(x) = node {
        position += 1;
        
        let left_node = &x.borrow().left;
        let right_node = &x.borrow().right;

        let left_farthest_position = get_farthest_level(left_node, position);
        let right_farthest_position = get_farthest_level(right_node, position);
        
        if  left_farthest_position > right_farthest_position  {
            position = left_farthest_position;
        } else {
            position = right_farthest_position;
        }
    }

    position
}