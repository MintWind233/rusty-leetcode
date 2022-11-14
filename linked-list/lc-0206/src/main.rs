// #[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

type Node = Option<Box<ListNode>>;

pub fn vec2list(testvec: Vec<i32>) -> Node {
    let mut head = None;
    for item in testvec.into_iter().rev() {
        let mut node = ListNode::new(item);
        node.next = head;
        head = Some(Box::new(node));
    }
    head
}

pub fn list2vec(mut head: Node) -> Vec<i32> {
    let mut res = vec![];
    let mut root = &mut head;
    while let Some(node) = root {
        res.push(node.val);
        root = &mut node.next;
    }
    res
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

struct Solution;

impl Solution {
    pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut lhs = head;
        let mut rhs = None;

        while let Some(mut node) = lhs {
            lhs = node.next.take();
            node.next = rhs;
            rhs = Some(node);
        }

        rhs
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(
            list2vec(Solution::reverse_list(vec2list(vec![1,2,3]))),
            list2vec(vec2list(vec![3,2,1]))
        );
    }
}
