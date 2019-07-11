// q0024_swap_nodes_in_pairs

struct Solution;

use crate::util::ListNode;

impl Solution {
    pub fn swap_pairs(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut list = head;
        let mut buf = vec![];
        while let Some(l) = list {
            buf.push(l.val);
            list = l.next;
        }
        if buf.len() < 1 {
            return None;
        }
        let mut i = 0;
        while i < buf.len() - 1 {
            let t = buf[i];
            buf[i] = buf[i + 1];
            buf[i + 1] = t;
            i += 2;
        }

        let len = buf.len();
        let mut ret = None;
        for i in 1..=len {
            ret = Some(Box::new(ListNode {
                val: buf[len - i],
                next: ret,
            }))
        }
        ret
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::util::ListNode;

    #[test]
    fn it_works() {
        assert_eq!(
            Solution::swap_pairs(ListNode::build(vec![1, 2, 3, 4])),
            ListNode::build(vec![2, 1, 4, 3])
        )
    }
}
