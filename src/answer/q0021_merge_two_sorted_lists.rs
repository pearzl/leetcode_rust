// q0021_merge_two_sorted_lists

struct Solution;

use crate::util::ListNode;

impl Solution {
    pub fn merge_two_lists(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let buf = Solution::mtl(l1, l2, vec![]);
        let mut ret = None;
        let len = buf.len();
        for i in 1..=len {
            ret = Some(Box::new(ListNode {
                val: buf[len - i],
                next: ret,
            }))
        }
        ret
    }

    fn mtl(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>, mut v: Vec<i32>) -> Vec<i32> {
        match (l1, l2) {
            (Some(l1), Some(l2)) => {
                if l1.val < l2.val {
                    v.push(l1.val);
                    return Solution::mtl(l1.next, Some(l2), v);
                } else {
                    v.push(l2.val);
                    return Solution::mtl(l2.next, Some(l1), v);
                }
            }
            (Some(l), None) => {
                let mut li = Some(l);
                while let Some(l) = li {
                    v.push(l.val);
                    if let Some(n) = l.next {
                        li = Some(n)
                    } else {
                        break;
                    }
                }
                return v;
            }
            (None, Some(l)) => {
                let mut li = Some(l);
                while let Some(l) = li {
                    v.push(l.val);
                    if let Some(n) = l.next {
                        li = Some(n)
                    } else {
                        break;
                    }
                }
                return v;
            }
            (None, None) => return v,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::util::ListNode;

    #[test]
    fn it_works() {
        assert_eq!(
            ListNode::build(vec![1, 1, 2, 3, 4, 4]),
            Solution::merge_two_lists(
                ListNode::build(vec![1, 2, 4]),
                ListNode::build(vec![1, 3, 4])
            )
        );
    }
}
