// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

/// The idea is that the fast pointer moves at double the speed so when
/// the fast pointer gets to the end of the list at double the speed of
/// the slow pointer so the slow pointer rests at the middle.
pub fn middle_node(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut slow = head.as_ref();
    let mut fast = head.as_ref();

    while let Some(fast_node) = fast {
        if fast_node.next.is_none() {
            break;
        }

        slow = slow.unwrap().next.as_ref();
        fast = fast_node.next.as_ref().and_then(|node| node.next.as_ref());
    }

    slow.cloned()
}
