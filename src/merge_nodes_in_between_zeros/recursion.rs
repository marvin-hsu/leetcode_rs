use crate::data_struct::ListNode;

pub fn merge_nodes(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    merge_nodes_ref(&head)
}

pub fn merge_nodes_ref(head: &Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    match head {
        None => None,
        Some(node) => {
            if node.val == 0 {
                merge_nodes_ref(&node.next)
            } else {
                let nextNode = merge_nodes_ref(&node.next);
                if node.next.as_ref().unwrap().val == 0 {
                    Some(Box::new(ListNode {
                        val: node.val,
                        next: nextNode,
                    }))
                } else {
                    Some(Box::new(ListNode {
                        val: node.val + nextNode.as_ref().unwrap().val,
                        next: nextNode.unwrap().next,
                    }))
                }
            }
        }
    }
}
