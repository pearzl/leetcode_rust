// q0129_sum_root_to_leaf_numbers

struct Solution;

use crate::util::TreeNode;

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn sum_numbers(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if let Some(t) = root {
            let v = Solution::make_nums(t);
            v.iter().map(|x| x.parse::<i32>().unwrap()).sum()
        } else {
            0
        }
    }

    fn make_nums(tree: Rc<RefCell<TreeNode>>) -> Vec<String> {
        let t = tree.borrow();
        match (t.left.clone(), t.right.clone()) {
            (Some(lt), Some(rt)) => {
                let v1 = Solution::make_nums(lt);
                let v2 = Solution::make_nums(rt);
                let mut v = Vec::with_capacity(v1.len() + v2.len());
                for i in v1.iter() {
                    let t = format!("{}{}", t.val, *i);
                    v.push(t);
                }
                for i in v2.iter() {
                    let t = format!("{}{}", t.val, *i);
                    v.push(t);
                }
                // println!("{:?}", v);
                v
            }
            (Some(lt), None) => {
                let tlt = Solution::make_nums(lt);
                let v = tlt.iter().map(|i| format!("{}{}", t.val, i)).collect();
                // println!("{:?}", v);
                v
            }
            (None, Some(rt)) => {
                let trt = Solution::make_nums(rt);
                let v = trt.iter().map(|i| format!("{}{}", t.val, i)).collect();
                // println!("{:?}", v);
                v
            }
            (None, None) => {
                // println!("{:?}", vec![t.val]);
                vec![format!("{}", t.val)]
            }
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
            25,
            Solution::sum_numbers(TreeNode::build_with_str("[1,2,3]"))
        );
        assert_eq!(
            1026,
            Solution::sum_numbers(TreeNode::build_with_str("[4,9,0,5,1]"))
        );
        assert_eq!(
            6363,
            Solution::sum_numbers(TreeNode::build_with_str("[5,3,2,7,0,6,null,null,null,0]"))
        );
    }
}
