// q0083_remove_duplicates_from_sorted_list

struct Solution;

use crate::util::ListNode;

impl Solution {
    pub fn delete_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut tmp = vec![];
        let mut head = head;
        while let Some(ln) = head {
            tmp.push(ln.val);
            head = ln.next;
        }
        let tl = tmp.len();
        if tl == 0 {
            return None;
        } else if tl == 1 {
            return Solution::build(tmp);
        }
        let mut last = tmp[0];
        let mut ret = vec![];
        for i in 1..tl {
            if last != tmp[i] {
                ret.push(last);
                last = tmp[i]
            }
        }
        ret.push(last);
        Solution::build(ret)
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
            ListNode::build(vec![1, 2, 3, 4, 5]),
            Solution::delete_duplicates(ListNode::build(vec![1, 2, 3, 3, 4, 4, 5]))
        );
        assert_eq!(
            ListNode::build(vec![1]),
            Solution::delete_duplicates(ListNode::build(vec![1, 1, 1, 1, 1]))
        );
        assert_eq!(
            ListNode::build(vec![]),
            Solution::delete_duplicates(ListNode::build(vec![]))
        );
    }
}
