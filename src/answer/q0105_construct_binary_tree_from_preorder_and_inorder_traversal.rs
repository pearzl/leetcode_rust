// q0105_construct_binary_tree_from_preorder_and_inorder_traversal

struct Solution;

use crate::util::TreeNode;

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn build_tree(preorder: Vec<i32>, inorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        Solution::build_tree1(&preorder, &inorder)
    }

    pub fn build_tree1(preorder: &[i32], inorder: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
        if inorder.len() == 0 {
            return None;
        }
        // println!("{:?}----{:?}", preorder, inorder);
        let val_pos = {
            let mut count = 0;
            for (i, n) in inorder.iter().enumerate() {
                if n == &preorder[0] {
                    count = i;
                    break;
                }
            }
            count
        };
        Some(Rc::new(RefCell::new(TreeNode {
            val: preorder[0],
            left: Solution::build_tree1(&preorder[1..=val_pos], &inorder[0..val_pos]),
            right: Solution::build_tree1(&preorder[val_pos + 1..], &inorder[val_pos + 1..]),
        })))
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    use crate::util::TreeNode;

    #[test]
    fn it_works() {
        assert_eq!(
            TreeNode::build(vec![
                Some(3),
                Some(9),
                Some(20),
                None,
                None,
                Some(15),
                Some(7)
            ]),
            Solution::build_tree(vec![3, 9, 20, 15, 7], vec![9, 3, 15, 20, 7])
        );
    }
}
