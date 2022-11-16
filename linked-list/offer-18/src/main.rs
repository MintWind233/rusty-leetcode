use ulti::linked_list::*;
pub struct Solution;

impl Solution {
    pub fn delete_node(head: Option<Box<ListNode>>, val: i32) -> Option<Box<ListNode>> {
        let mut head = head;
        if let Some(node) = &head {
            if node.val == val {
                head = head.unwrap().next;
            }
        }
        let mut root = &mut head;
        while let Some(node) = root {
            let next_node = &mut node.next;
            if let Some(t) = next_node {
                if t.val == val {
                    node.next = t.next.take();
                }
            }
            root = &mut node.next;
        }
        head
    }
}

#[cfg(test)]
mod test{
    use super::*;
    use ulti::linked_list::ListNode;

    #[test]
    pub fn it_works() {
        assert_eq!(
            ListNode::list2vec(Solution::delete_node(ListNode::vec2list(vec![4,5,1,9]), 5)),
            vec![4,1,9]
        );
        assert_eq!(
            ListNode::list2vec(Solution::delete_node(ListNode::vec2list(vec![4,5,1,9]), 1)),
            vec![4,5,9]
        );
        assert_eq!(
            ListNode::list2vec(Solution::delete_node(ListNode::vec2list(vec![1,3,4]), 1)),
            vec![3,4]
        )
    }
}

fn main() {
    println!("{:?}", ListNode::list2vec(Solution::delete_node(ListNode::vec2list(vec![1,4,5,6,8,10]), 4)));
}
