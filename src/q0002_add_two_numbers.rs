// q0002_add_two_numbers

use crate::util::ListNode;

struct Solution;

impl Solution {
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut ret = vec![];
        let mut flag = 0;
        let mut l1 = l1;
        let mut l2 = l2;
        let mut output = None;
        loop {
            if l1 == None && l2 == None {
                break;
            }
            let c1 = if l1 == None {
                0
            } else {
                let t1 = l1.unwrap();
                l1 = t1.next;
                t1.val
            };
            let c2 = if l2 == None {
                0
            } else {
                let t2 = l2.unwrap();
                l2 = t2.next;
                t2.val
            };
            let mut n = c1 + c2 + flag;
            if n >= 10 {
                n -= 10;
                flag = 1;
            } else {
                flag = 0;
            }
            ret.push(n)
        }
        if flag == 1 {
            ret.push(1)
        }
        while let Some(n) = ret.pop() {
            output = Some(Box::new(ListNode {
                val: n,
                next: output,
            }));
        }
        output
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::util::ListNode;

    #[test]
    fn it_works() {
        assert_eq!(
            Solution::add_two_numbers(
                ListNode::build(vec![2, 4, 3]),
                ListNode::build(vec![5, 6, 4])
            ),
            ListNode::build(vec![7, 0, 8])
        );
    }
}
