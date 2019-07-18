// q0086_partition_list

struct Solution;

use crate::util::ListNode;

impl Solution {
    pub fn partition(head: Option<Box<ListNode>>, x: i32) -> Option<Box<ListNode>> {
        let mut tmp = vec![];
        let mut head = head;
        while let Some(ln) = head {
            tmp.push(ln.val);
            head = ln.next;
        }

        let mut k = 0;
        for i in 0..tmp.len() {
            if tmp[i] < x {
                let t = tmp.remove(i);
                tmp.insert(k, t);
                k += 1;
            }
        }

        Solution::build(tmp)
    }

    pub fn build(v: Vec<i32>) -> Option<Box<ListNode>> {
        let mut ret = None;
        for i in 1..=v.len() {
            ret = Some(Box::new(ListNode {
                val: v[v.len() - i],
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
            Solution::partition(ListNode::build(vec![1, 4, 3, 2, 5, 2]), 3),
            ListNode::build(vec![1, 2, 2, 4, 3, 5])
        );
    }
}
