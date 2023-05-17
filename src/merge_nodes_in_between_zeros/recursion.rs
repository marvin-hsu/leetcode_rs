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
                let next_node = merge_nodes_ref(&node.next);
                if node.next.as_ref().unwrap().val == 0 {
                    Some(Box::new(ListNode {
                        val: node.val,
                        next: next_node,
                    }))
                } else {
                    Some(Box::new(ListNode {
                        val: node.val + next_node.as_ref().unwrap().val,
                        next: next_node.unwrap().next,
                    }))
                }
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_merge_nodes_1() {
        let input = create_linked_list(vec![0, 3, 1, 0, 4, 5, 2, 0]);
        let expected_output = create_linked_list(vec![4, 11]);

        assert_eq!(merge_nodes(input), expected_output);
    }

    #[test]
    fn test_merge_nodes_2() {
        let input = create_linked_list(vec![0, 1, 0, 3, 0, 2, 2, 0]);
        let expected_output = create_linked_list(vec![1, 3, 4]);

        assert_eq!(merge_nodes(input), expected_output);
    }

    // Helper function to create a linked list from a vector of values
    fn create_linked_list(values: Vec<i32>) -> Option<Box<ListNode>> {
        let mut head = None;
        for &val in values.iter().rev() {
            let mut node = ListNode::new(val);
            node.next = head;
            head = Some(Box::new(node));
        }
        head
    }
}
