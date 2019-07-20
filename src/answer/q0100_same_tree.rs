// q0100_same_tree

struct Solution;

use crate::util::TreeNode;

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn is_same_tree(
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        p == q
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    use crate::util::TreeNode;

    #[test]
    fn it_works() {
        assert_eq!(
            true,
            Solution::is_same_tree(
                TreeNode::build(vec![Some(1), Some(2), Some(3)]),
                TreeNode::build(vec![Some(1), Some(2), Some(3)])
            )
        );
        assert_eq!(
            false,
            Solution::is_same_tree(
                TreeNode::build(vec![Some(1), Some(2)]),
                TreeNode::build(vec![Some(1), None, Some(3)])
            )
        );
    }
}
