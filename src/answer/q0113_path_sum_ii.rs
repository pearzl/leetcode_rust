// q0113_path_sum_ii

struct Solution;

use crate::util::TreeNode;

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn path_sum(root: Option<Rc<RefCell<TreeNode>>>, sum: i32) -> Vec<Vec<i32>> {
        let mut tvd = vec![];
        let mut ret = vec![];
        Solution::path_traveral(&mut tvd, sum, &mut ret, root);
        ret
    }

    fn path_traveral(
        tvd: &mut Vec<i32>,
        sum: i32,
        ret: &mut Vec<Vec<i32>>,
        tree: Option<Rc<RefCell<TreeNode>>>,
    ) {
        if tree.is_none() {
            return;
        }
        let tree = tree.unwrap();
        let tn = tree.borrow();
        match (&tn.left, &tn.right) {
            (Some(ln), Some(rn)) => {
                let new_sum = sum - tn.val;
                let new_tree1 = Some(Rc::clone(ln));
                tvd.push(tn.val);
                Solution::path_traveral(tvd, new_sum, ret, new_tree1);
                tvd.pop();
                let new_tree2 = Some(Rc::clone(rn));
                tvd.push(tn.val);
                Solution::path_traveral(tvd, new_sum, ret, new_tree2);
                tvd.pop();
            }
            (Some(ln), None) => {
                let new_sum = sum - tn.val;
                let new_tree = Some(Rc::clone(ln));
                tvd.push(tn.val);
                Solution::path_traveral(tvd, new_sum, ret, new_tree);
                tvd.pop();
            }
            (None, Some(rn)) => {
                let new_sum = sum - tn.val;
                let new_tree = Some(Rc::clone(rn));
                tvd.push(tn.val);
                Solution::path_traveral(tvd, new_sum, ret, new_tree);
                tvd.pop();
            }
            (None, None) => {
                if sum == tn.val {
                    let mut t = tvd.clone();
                    t.push(sum);
                    ret.push(t);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    use crate::util::{self, TreeNode};

    #[test]
    fn it_works() {
        assert_eq!(
            util::vec_2_set(vec![vec![5, 4, 11, 2], vec![5, 8, 4, 5]]),
            util::vec_2_set(Solution::path_sum(
                TreeNode::build_with_str("[5,4,8,11,null,13,4,7,2,null,null,5,1]"),
                22
            ))
        );
    }
}
