use ulti::linked_list::ListNode;

pub struct Solution;

impl Solution {
    pub fn delete_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        if head.is_none() || head.as_ref()?.next.is_none() {
            return head;
        }
        let mut head = head;
        let mut root = &mut head;

        while root.is_some() && root.as_mut()?.next.is_some() {
            let mut node = root.as_mut().unwrap();
            let next_node = &mut node.next;
            if next_node.as_ref()?.val == node.val {
                node.next = next_node.as_mut()?.next.take();
            } else {
                root = &mut root.as_mut()?.next;
            }
        }

        head
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn delete_duplicates_works() {
        assert_eq!(
            ListNode::list2vec(Solution::delete_duplicates(ListNode::vec2list(vec![
                1, 1, 2
            ]))),
            vec![1, 2]
        );
        assert_eq!(
            ListNode::list2vec(Solution::delete_duplicates(ListNode::vec2list(vec![
                1, 1, 2, 3, 3
            ]))),
            vec![1, 2, 3]
        );
    }
}

fn main() {
    println!("Hello, world!");
}
