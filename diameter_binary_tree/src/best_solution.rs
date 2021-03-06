use std::rc::Rc;
use std::cell::RefCell;
use std::cmp::max;

type Depth = i32;
type Diameter = i32;
type Node = Option<Rc<RefCell<TreeNode>>>;

impl Solution {
    pub fn diameter_of_binary_tree(root: Node) -> i32 {
        Solution::depth_and_diameter(&root).1
    }

    fn depth_and_diameter(root: &Node) -> (Depth, Diameter) {
        if let Some(node) = root {
            let (l_depth, l_diameter) = Solution::depth_and_diameter(&node.borrow().left);
            let (r_depth, r_diameter) = Solution::depth_and_diameter(&node.borrow().right);
            (max(l_depth, r_depth)+1, max(l_depth+r_depth, max(l_diameter, r_diameter)))
        } else {
            (0, 0)
        }
    }
}