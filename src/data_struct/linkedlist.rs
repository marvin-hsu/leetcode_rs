#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[allow(dead_code)]
    pub fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }

    // Helper function to create a linked list from a vector of values
    #[allow(dead_code)]
    pub fn create_linked_list(values: Vec<i32>) -> Option<Box<ListNode>> {
        let mut head = None;
        for &val in values.iter().rev() {
            let mut node = ListNode::new(val);
            node.next = head;
            head = Some(Box::new(node));
        }
        head
    }
}
