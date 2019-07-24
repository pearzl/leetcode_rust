// q0112_path_sum

struct Solution;

use crate::util::TreeNode;

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn has_path_sum(root: Option<Rc<RefCell<TreeNode>>>, sum: i32) -> bool {
        if let Some(rrc_t) = root {
            let tn = rrc_t.borrow();
            match (&tn.left, &tn.right) {
                (Some(ln), Some(rn)) => {
                    let new_sum = sum - tn.val;
                    let new_tree1 = Some(Rc::clone(rn));
                    let new_tree2 = Some(Rc::clone(ln));
                    return Solution::has_path_sum(new_tree1, new_sum)
                        || Solution::has_path_sum(new_tree2, new_sum);
                }
                (Some(ln), None) => {
                    let new_sum = sum - tn.val;
                    let new_tree = Some(Rc::clone(ln));
                    return Solution::has_path_sum(new_tree, new_sum);
                }
                (None, Some(rn)) => {
                    let new_sum = sum - tn.val;
                    let new_tree = Some(Rc::clone(rn));
                    return Solution::has_path_sum(new_tree, new_sum);
                }
                (None, None) => return tn.val == sum,
            }
        } else {
            return false;
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
            true,
            Solution::has_path_sum(
                TreeNode::build_with_str("[5,4,8,11,null,13,4,7,2,null,null,null,1]"),
                22
            )
        );
    }
}
