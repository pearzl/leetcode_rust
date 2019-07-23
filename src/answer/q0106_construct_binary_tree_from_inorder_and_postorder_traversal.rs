// q0106_construct_binary_tree_from_inorder_and_postorder_traversal

struct Solution;

use crate::util::TreeNode;

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn build_tree(inorder: Vec<i32>, postorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        Solution::build_tree1(&inorder, &postorder)
    }

    pub fn build_tree1(inorder: &[i32], postorder: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
        let post_len = postorder.len();
        if post_len == 0 {
            return None;
        }
        // println!("{:?}----{:?}", inorder, postorder);
        let val_pos = {
            let mut count = 0;
            for (i, n) in inorder.iter().enumerate() {
                if n == &postorder[post_len - 1] {
                    count = i;
                    break;
                }
            }
            count
        };
        Some(Rc::new(RefCell::new(TreeNode {
            val: postorder[post_len - 1],
            left: Solution::build_tree1(&inorder[0..val_pos], &postorder[0..val_pos]),
            right: Solution::build_tree1(
                &inorder[val_pos + 1..],
                &postorder[val_pos..post_len - 1],
            ),
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
            Solution::build_tree(vec![9, 3, 15, 20, 7], vec![9, 15, 7, 20, 3])
        );
    }
}
