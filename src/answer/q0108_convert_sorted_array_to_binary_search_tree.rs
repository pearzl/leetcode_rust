// q0108_convert_sorted_array_to_binary_search_tree

struct Solution;

use crate::util::TreeNode;

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn sorted_array_to_bst(nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        Solution::sorted_array_to_bst1(&nums)
    }

    pub fn sorted_array_to_bst1(nums: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
        let nlen = nums.len();
        if nlen == 0 {
            return None;
        }
        let mid = nlen / 2;
        Some(Rc::new(RefCell::new(TreeNode {
            val: nums[mid],
            left: Solution::sorted_array_to_bst1(&nums[0..mid]),
            right: Solution::sorted_array_to_bst1(&nums[mid + 1..]),
        })))
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    use crate::util::TreeNode;

    #[test]
    fn it_works() {
        // assert_eq!( TreeNode::build(vec![Some(0), Some(-10), Some(5), None, Some(-3), None, Some(9)]), Solution::sorted_array_to_bst(vec![-10,-3,0,5,9]));
        assert_eq!(
            TreeNode::build(vec![Some(0), Some(-3), Some(9), Some(-10), None, Some(5)]),
            Solution::sorted_array_to_bst(vec![-10, -3, 0, 5, 9])
        );
    }
}
