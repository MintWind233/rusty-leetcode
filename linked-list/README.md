### 链表结构

```Rust
// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}

impl ListNode {
  #[inline]
  fn new(val: i32) -> Self {
    ListNode {
      next: None,
      val
    }
  }
}
impl Solution {
    pub fn is_palindrome(head: Option<Box<ListNode>>) -> bool {

    }
}
```

#### 遍历链表

```rust
let mut root = &mut head;
while let Some(node) = root {
  let next_node = &mut node.next;
  // 使用as_mut获取next_node的引用，使用&mut获取.next的引用。以此来获取root下一个节点的下一个节点的引用。直接使用unwrap会导致所有权的move
  let next_node_next = &mut next_node.as_mut()?.next;
  // 这里面不能再直接使用head，因为head的所有权已经借给了root，在循环体中未归还
  // other code...
  root = &mut node.next;
}



while head.as_mut()?.next.is_some() {
    head = &mut head.as_mut()?.next;
}
```

#### 转移获取链表所有权

```rust
let next_node = node.next.take()
```

#### ulti测试函数之vec2List

```rust
// 反着创建会比较方便，正好返回尾节点就是头节点
pub fn vec2list(testvec: Vec<i32>) -> Option<Box<List>> {
    let mut head = None;
    for item in testvec.into_iter().rev() {
        let mut node = ListNode::new(item);
        node.next = head;
        head = Some(Box::new(node));
    }
    head
}
```

