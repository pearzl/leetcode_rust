// q0025_reverse_nodes_in_k_group

struct Solution;

use crate::util::ListNode;

impl Solution {
    pub fn reverse_k_group(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        let mut tmp = vec![];
        let mut buf = vec![];
        let mut count = 0;
        let mut head = head;
        while let Some(l) = head {
            tmp.push(l.val);
            head = l.next;
            count += 1;
            if count == k {
                buf.push(tmp.clone());
                tmp.clear();
                count = 0;
            }
        }
        if tmp.len() != 0 {
            buf.push(tmp);
        }
        let mut ret = vec![];
        for i in buf.iter_mut() {
            if i.len() != k as usize {
                ret.append(&mut *i)
            } else {
                while let Some(item) = i.pop() {
                    ret.push(item);
                }
            }
        }
        build(ret)
    }
}
fn build(v: Vec<i32>) -> Option<Box<ListNode>> {
    let mut ret = None;
    for i in 1..=v.len() {
        ret = Some(Box::new(ListNode {
            val: v[v.len() - i],
            next: ret,
        }))
    }
    ret
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn it_works() {
        assert_eq!(
            Solution::reverse_k_group(super::build(vec![1, 2, 3, 4, 5]), 2),
            super::build(vec![2, 1, 4, 3, 5])
        );
        assert_eq!(
            Solution::reverse_k_group(super::build(vec![1, 2, 3, 4, 5]), 3),
            super::build(vec![3, 2, 1, 4, 5])
        );
    }
}
