// q0109_convert_sorted_list_to_binary_search_tree

struct Solution;

use crate::util::{ListNode, TreeNode};

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn sorted_list_to_bst(head: Option<Box<ListNode>>) -> Option<Rc<RefCell<TreeNode>>> {
        let mut nums = vec![];
        let mut head = head;
        while let Some(ln) = head {
            nums.push(ln.val);
            head = ln.next;
        }
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

    use crate::util::{ListNode, TreeNode};

    #[test]
    fn it_works() {
        assert_eq!(
            TreeNode::build(vec![Some(0), Some(-3), Some(9), Some(-10), None, Some(5)]),
            Solution::sorted_list_to_bst(ListNode::build(vec![-10, -3, 0, 5, 9]))
        );
    }
}
