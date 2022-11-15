pub mod linked_list {
    // type alias
    type NodePtr = Option<Box<ListNode>>;

    // standard Leetcode struct
    #[derive(PartialEq, Eq, Debug, Clone)]
    pub struct ListNode {
        pub val: i32,
        pub next: NodePtr,
    }

    impl ListNode {
        // Constructor function
        pub fn new(val: i32) -> Self {
            ListNode { next: None, val }
        }

        // Vector to LinkedList
        pub fn vec2list(fromvec: Vec<i32>) -> NodePtr {
            let mut head = None;
            for elem in fromvec.into_iter().rev() {
                let mut node = Self::new(elem);
                node.next = head;
                head = Some(Box::new(node));
            }
            head
        }

        // List to LinedList
        pub fn list2vec(mut head: NodePtr) -> Vec<i32> {
            let mut res = vec![];
            let mut root = &mut head;
            while let Some(node) = root {
                res.push(node.val);
                root = &mut node.next;
            }
            res
        }

        // compare two list
        pub fn compare_list(list1: NodePtr, list2: NodePtr) -> bool {
            Self::list2vec(list1) == Self::list2vec(list2)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::linked_list::*;

    #[test]
    fn to_lists() {}
}
