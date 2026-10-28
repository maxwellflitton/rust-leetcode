type NodePtr = Option<Box<ListNode>>;

#[derive(Clone)]
struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

fn niave_rotate_right(head: NodePtr, k: i32) -> NodePtr {
    // do early exit if the head is None or k is one
    if head.is_none() || k == 0 {
        return head;
    }

    // construct the values in an array
    let mut buffer = Vec::new();

    let mut ptr = head.as_ref();
    while let Some(node) = ptr {
        buffer.push(node.val);
        ptr = node.next.as_ref();
    }

    // Calculate where the rotated list starts
    let n = buffer.len();
    let starting_point = (n - (k as usize % n)) % n;

    // Construct the first node
    let mut head = Box::new(ListNode {
        val: buffer[starting_point],
        next: None,
    });

    // Borrow the tail, don't take ownership
    let mut current_node = &mut head;
    let mut buffer_ptr = (starting_point + 1) % n;

    while buffer_ptr != starting_point {
        current_node.next = Some(Box::new(ListNode {
            val: buffer[buffer_ptr],
            next: None,
        }));

        // Move the mutable reference to the new tail
        current_node = current_node.next.as_mut().unwrap();

        buffer_ptr = (buffer_ptr + 1) % n;
    }

    Some(head)
}

fn optimized_rotate_right(mut head: NodePtr, k: i32) -> NodePtr {
    if head.is_none() || k == 0 {
        return head;
    }

    // First pass: calculate length
    let mut len = 0;
    let mut ptr = head.as_ref();

    while let Some(node) = ptr {
        len += 1;
        ptr = node.next.as_ref();
    }

    // Normalize the rotation
    let k = k as usize % len;

    if k == 0 {
        return head;
    }

    // The new tail is at index len - k - 1
    let steps = len - k - 1;

    // Second pass: find the new tail
    let mut slow = head.as_mut().unwrap();

    for _ in 0..steps {
        slow = slow.next.as_mut().unwrap();
    }

    // Cut the list after the new tail
    let mut new_head = slow.next.take();

    // Find the old tail, now at the end of new_head
    let mut tail = new_head.as_mut().unwrap();

    while tail.next.is_some() {
        tail = tail.next.as_mut().unwrap();
    }

    // Connect old tail to original head
    tail.next = head;

    new_head
}

#[cfg(test)]
mod tests {

    use super::*;
}
