use ulti::linked_list::ListNode;
pub struct Solution;

impl Solution {
    pub fn middle_node(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let (mut slow, mut fast) = (&head, &head);
        while fast.as_ref().is_some() && fast.as_ref()?.next.is_some() {
            slow = &slow.as_ref()?.next;
            fast = &fast.as_ref()?.next.as_ref()?.next;
        }
        slow.clone()
    }
}


fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(
            ListNode::list2vec(Solution::middle_node(ListNode::vec2list(vec![1,2,3,4,5]))),
            vec![3,4,5]
        );
        assert_eq!(
            ListNode::list2vec(Solution::middle_node(ListNode::vec2list(vec![1,2,3,4,5,6]))),
            vec![4,5,6]
        )
    }
}
