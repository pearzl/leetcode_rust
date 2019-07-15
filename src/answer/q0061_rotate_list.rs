// q0061_rotate_list

struct Solution;

use crate::util::ListNode;

impl Solution {
    pub fn rotate_right(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        if k == 0 {
            return head;
        }
        if head.is_none() {
            return head;
        }
        let mut head = head;
        let mut tmp = vec![];
        while let Some(ln) = head {
            tmp.push(ln.val);
            head = ln.next;
        }
        let mut ret = vec![];
        let l = tmp.len();
        let k = k as usize % l;
        for i in &tmp[l - k..] {
            ret.push(*i);
        }
        for i in &tmp[0..l - k] {
            ret.push(*i);
        }
        ListNode::build(ret)
        // build(ret)
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    use crate::util::ListNode;

    #[test]
    fn it_works() {
        assert_eq!(
            ListNode::build(vec![4, 5, 1, 2, 3]),
            Solution::rotate_right(ListNode::build(vec![1, 2, 3, 4, 5]), 2)
        );
        assert_eq!(
            ListNode::build(vec![3, 1, 2]),
            Solution::rotate_right(ListNode::build(vec![1, 2, 3]), 4)
        );
        assert_eq!(
            ListNode::build(vec![3, 1, 2]),
            Solution::rotate_right(ListNode::build(vec![3, 1, 2]), 0)
        );
        assert_eq!(
            ListNode::build(vec![]),
            Solution::rotate_right(ListNode::build(vec![]), 0)
        );
    }
}
