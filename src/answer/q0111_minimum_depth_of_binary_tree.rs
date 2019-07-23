// q0111_minimum_depth_of_binary_tree

struct Solution;

use crate::util::TreeNode;

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn min_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if let Some(rrc) = root {
            match (&rrc.borrow().left, &rrc.borrow().right) {
                (Some(ln), Some(rn)) => {
                    let d1 = Solution::min_depth(Some(Rc::clone(ln)));
                    let d2 = Solution::min_depth(Some(Rc::clone(rn)));
                    return 1 + d1.min(d2);
                }
                (Some(ln), None) => return 1 + Solution::min_depth(Some(Rc::clone(ln))),
                (None, Some(rn)) => return 1 + Solution::min_depth(Some(Rc::clone(rn))),
                (None, None) => return 1,
            }
        } else {
            return 0;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    use crate::util::TreeNode;

    #[test]
    fn it_works() {
        assert_eq!(
            2,
            Solution::min_depth(TreeNode::build(vec![
                Some(3),
                Some(9),
                Some(20),
                None,
                None,
                Some(15),
                Some(7)
            ]))
        );
    }
}
