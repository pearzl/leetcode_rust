// q0124_binary_tree_maximum_path_sum

struct Solution;

use crate::util::TreeNode;

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn max_path_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if let Some(rrct_root) = root {
            let max_tree = Solution::max_value_tree(rrct_root.clone());
            println!("{:?}", Solution::into_vec(Some(max_tree.clone())));
            println!("{:?}", Solution::into_vec(Some(rrct_root.clone())));
            let arr = Solution::into_vec(Some(max_tree));
            arr.into_iter()
                .filter(|x| x.is_some())
                .map(|x| x.unwrap())
                .max()
                .unwrap()
        } else {
            0
        }
    }

    pub fn into_vec(tree: Option<Rc<RefCell<TreeNode>>>) -> Vec<Option<i32>> {
        use std::collections::VecDeque;

        let mut queue = VecDeque::new();
        queue.push_back(tree);
        let mut ret = vec![];
        while let Some(node) = queue.pop_front() {
            match node {
                None => {
                    ret.push(None);
                }
                Some(t) => {
                    ret.push(Some(t.borrow().val));
                    if let Some(ref lt) = t.borrow().left {
                        queue.push_back(Some(Rc::clone(lt)));
                    } else {
                        queue.push_back(None);
                    }
                    if let Some(ref rt) = t.borrow().right {
                        queue.push_back(Some(Rc::clone(rt)));
                    } else {
                        queue.push_back(None);
                    }
                }
            }
        }
        while let Some(item) = ret.pop() {
            if let Some(t) = item {
                ret.push(Some(t));
                break;
            }
        }
        ret
    }

    pub fn max_value_tree(root: Rc<RefCell<TreeNode>>) -> Rc<RefCell<TreeNode>> {
        let mut root_tree = root.borrow_mut();
        let mut new_tree = TreeNode::new(root_tree.val);
        match (root_tree.left.clone(), root_tree.right.clone()) {
            (Some(rrct_lt), Some(rrct_rt)) => {
                new_tree.left = Some(Solution::max_value_tree(rrct_lt.clone()));
                new_tree.right = Some(Solution::max_value_tree(rrct_rt.clone()));
                let t1 = root_tree.val + rrct_rt.borrow().val;
                let t2 = root_tree.val + rrct_lt.borrow().val;
                let t3 = root_tree.val + rrct_lt.borrow().val + rrct_rt.borrow().val;
                root_tree.val = root_tree.val.max(t1).max(t2);
                // println!("{}-{}-{}-->{}", t1, t2, t3, root_tree.val);
                new_tree.val = new_tree.val.max(t1).max(t2).max(t3);
                // println!("{}-{:?}-{:?}-->{}", new_tree.val, new_tree.left.clone(), new_tree.right.clone(), new_tree.val);
            }
            (Some(rrct_lt), None) => {
                new_tree.left = Some(Solution::max_value_tree(rrct_lt.clone()));
                root_tree.val = root_tree.val.max(root_tree.val + rrct_lt.borrow().val);
                new_tree.val = new_tree.val.max(new_tree.val + rrct_lt.borrow().val);
            }
            (None, Some(rrct_rt)) => {
                new_tree.right = Some(Solution::max_value_tree(rrct_rt.clone()));
                root_tree.val = root_tree.val.max(root_tree.val + rrct_rt.borrow().val);
                new_tree.val = new_tree.val.max(new_tree.val + rrct_rt.borrow().val);
            }
            (None, None) => {}
        }
        Rc::new(RefCell::new(new_tree))
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    use crate::util::TreeNode;

    #[test]
    fn it_works() {
        assert_eq!(
            6,
            Solution::max_path_sum(TreeNode::build_with_str("[1,2,3]"))
        );
        assert_eq!(
            42,
            Solution::max_path_sum(TreeNode::build_with_str("[-10,9,20,null,null,15,7]"))
        );
        assert_eq!(-1, Solution::max_path_sum(TreeNode::build_with_str("[-1]")));
        assert_eq!(
            48,
            Solution::max_path_sum(TreeNode::build_with_str(
                "[5,4,8,11,null,13,4,7,2,null,null,null,1]"
            ))
        );
    }

}
