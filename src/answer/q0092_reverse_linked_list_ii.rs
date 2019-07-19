// q0092_reverse_linked_list_ii

struct Solution;

use crate::util::ListNode;

impl Solution {
    pub fn reverse_between(head: Option<Box<ListNode>>, m: i32, n: i32) -> Option<Box<ListNode>> {
        let mut tmp = vec![];
        let mut head = head;
        while let Some(ln) = head {
            tmp.push(ln.val);
            head = ln.next;
        }
        let mut i = m as usize - 1;
        let mut j = n as usize - 1;
        while i < j {
            let t = tmp[i];
            tmp[i] = tmp[j];
            tmp[j] = t;
            j -= 1;
            i += 1;
        }
        let mut ret = None;
        for i in 1..=tmp.len() {
            ret = Some(Box::new(ListNode {
                val: tmp[tmp.len() - i],
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
            ListNode::build(vec![1, 4, 3, 2, 5]),
            Solution::reverse_between(ListNode::build(vec![1, 2, 3, 4, 5]), 2, 4)
        );
        assert_eq!(
            ListNode::build(vec![1, 3, 2, 4, 5]),
            Solution::reverse_between(ListNode::build(vec![1, 2, 3, 4, 5]), 2, 3)
        );
        assert_eq!(
            ListNode::build(vec![1, 2, 3, 4, 5]),
            Solution::reverse_between(ListNode::build(vec![1, 2, 3, 4, 5]), 2, 2)
        );
    }
}
