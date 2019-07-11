// q0019_remove_nth_node_from_end_of_list

struct Solution;

use crate::util::ListNode;

impl Solution {
    pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        let mut buf = vec![];
        let mut node = head;
        while let Some(l) = node {
            buf.push(l.val);
            node = l.next;
        }
        let mut ret = None;
        let len = buf.len();
        let mut count = 0;
        for i in 1..=len {
            count += 1;
            if count == n {
                continue;
            }
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
            Solution::remove_nth_from_end(
                Some(Box::new(ListNode {
                    val: 1,
                    next: Some(Box::new(ListNode {
                        val: 2,
                        next: Some(Box::new(ListNode {
                            val: 3,
                            next: Some(Box::new(ListNode {
                                val: 4,
                                next: Some(Box::new(ListNode { val: 5, next: None }))
                            }))
                        }))
                    }))
                })),
                2
            ),
            Some(Box::new(ListNode {
                val: 1,
                next: Some(Box::new(ListNode {
                    val: 2,
                    next: Some(Box::new(ListNode {
                        val: 3,
                        next: Some(Box::new(ListNode { val: 5, next: None }))
                    }))
                }))
            }))
        )
    }
}
