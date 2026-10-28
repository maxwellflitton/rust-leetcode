#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    #[allow(dead_code)]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

pub fn reverse_between(
    head: Option<Box<ListNode>>,
    left: i32,
    right: i32,
) -> Option<Box<ListNode>> {
    let mut dummy = Box::new(ListNode { val: 0, next: head });

    let mut before = &mut dummy;

    // Stop immediately before the section being reversed.
    for _ in 1..left {
        before = before.next.as_mut().unwrap();
    }

    // Take ownership of the section.
    let mut current = before.next.take();
    let mut reversed: Option<Box<ListNode>> = None;

    // Reverse exactly right - left + 1 nodes. the `for _ in 0..(right - left + 1)` performs the same number of iterations
    for _ in left..=right {
        let mut node = current.unwrap();
        current = node.next.take();

        node.next = reversed;
        reversed = Some(node);
    }

    // Find the tail of the reversed section and attach the remainder.
    let mut tail = reversed.as_mut().unwrap();

    while tail.next.is_some() {
        tail = tail.next.as_mut().unwrap();
    }

    tail.next = current;

    // Reconnect the reversed section to the prefix.
    before.next = reversed;

    dummy.next
}

/// Remove duplicates from a **sorted** linked list so that each value appears
/// only once (LeetCode 83).
///
/// We keep a single cursor that always points at a node we are keeping. For
/// that node we skip over every following node that shares its value, then
/// advance the cursor. Because the list is sorted, all duplicates of a value
/// sit next to each other, so a single forward pass is enough.
pub fn delete_duplicates(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut current = head.as_mut();

    // iterate until the linked list is expired
    while let Some(node) = current {
        // inner loop that will keep going unitl the end of the
        // linked list is done
        while let Some(next) = node.next.as_ref() {
            if next.val == node.val {
                node.next = node.next.take().unwrap().next; // splice out the duplicate
            } else {
                break;
            }
        }

        // essentially increasing the pointer
        current = node.next.as_mut();
    }
    head
}

#[cfg(test)]
mod tests {
    use super::{ListNode, delete_duplicates};

    /// Build a linked list from a slice of values, preserving order.
    fn build(values: &[i32]) -> Option<Box<ListNode>> {
        let mut head: Option<Box<ListNode>> = None;
        // Iterate back-to-front so each new node points at the list built so far.
        for &val in values.iter().rev() {
            head = Some(Box::new(ListNode { val, next: head }));
        }
        head
    }

    /// Collect a linked list back into a `Vec` so it is easy to assert on.
    fn collect(mut head: Option<Box<ListNode>>) -> Vec<i32> {
        let mut out = Vec::new();
        while let Some(node) = head {
            out.push(node.val);
            head = node.next;
        }
        out
    }

    // ────────────────
    // Examples (from LeetCode 83)
    // ────────────────
    #[test]
    fn example_1() {
        let list = build(&[1, 1, 2]);
        assert_eq!(collect(delete_duplicates(list)), vec![1, 2]);
    }

    #[test]
    fn example_2() {
        let list = build(&[1, 1, 2, 3, 3]);
        assert_eq!(collect(delete_duplicates(list)), vec![1, 2, 3]);
    }

    // ────────────────
    // Tiny / edge inputs
    // ────────────────
    #[test]
    fn empty_list() {
        assert_eq!(collect(delete_duplicates(build(&[]))), Vec::<i32>::new());
    }

    #[test]
    fn single_element() {
        assert_eq!(collect(delete_duplicates(build(&[7]))), vec![7]);
    }

    #[test]
    fn no_duplicates() {
        assert_eq!(collect(delete_duplicates(build(&[1, 2, 3]))), vec![1, 2, 3]);
    }

    #[test]
    fn all_the_same() {
        assert_eq!(collect(delete_duplicates(build(&[5, 5, 5, 5]))), vec![5]);
    }

    #[test]
    fn duplicates_at_the_end() {
        assert_eq!(
            collect(delete_duplicates(build(&[1, 2, 3, 3]))),
            vec![1, 2, 3]
        );
    }

    #[test]
    fn negatives() {
        let list = build(&[-3, -3, -1, 0, 0, 2]);
        assert_eq!(collect(delete_duplicates(list)), vec![-3, -1, 0, 2]);
    }

    // ────────────────
    // Stress / large input
    // ────────────────
    #[test]
    fn large_input() {
        // Each value 0..n repeated three times, still sorted.
        let n = 10_000;
        let mut values = Vec::with_capacity(n * 3);
        for v in 0..n as i32 {
            values.extend_from_slice(&[v, v, v]);
        }

        let result = collect(delete_duplicates(build(&values)));

        assert_eq!(result.len(), n);
        assert_eq!(result.first(), Some(&0));
        assert_eq!(result.last(), Some(&(n as i32 - 1)));
        for (idx, val) in result.iter().enumerate() {
            assert_eq!(*val, idx as i32);
        }
    }
}
