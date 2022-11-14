use std::collections::VecDeque;

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub next: Option<Box<ListNode>>,
    pub val: i32,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

type Node = Option<Box<ListNode>>;

struct Solution;

impl Solution {
    pub fn is_palindrome(mut head: Option<Box<ListNode>>) -> bool {
        // 双端队列Deque存放数据
        let mut deque = VecDeque::new();
        let mut root = &mut head;
        while let Some(node) = root {
            let value = node.val;
            deque.push_back(value);
            root = &mut node.next;
        }
        while deque.len()>1 {
            if deque.pop_back().unwrap() != deque.pop_front().unwrap() {
                return false;
            }
        }
        true
    }
}

fn main() {
    
}

#[cfg(test)]
mod tests {
    use super::*;

    // rust链表是反着排的
    pub fn vec2list(testvec: Vec<i32>) -> Node {
        let mut head = None;
        for item in testvec.into_iter().rev() {
            let mut node = ListNode::new(item);
            node.next = head;
            head = Some(Box::new(node));
        }
        head
    }

    #[test]
    fn func_works() {
        assert_eq!(Solution::is_palindrome(vec2list(vec![1,2,1])), true);
        assert_eq!(Solution::is_palindrome(vec2list(vec![1,2,3,3,2,1])), true);
        assert_eq!(Solution::is_palindrome(vec2list(vec![1,2])), false);
    }
}
