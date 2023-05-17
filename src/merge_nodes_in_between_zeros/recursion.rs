use crate::data_struct::ListNode;

#[allow(dead_code)]
pub fn merge_nodes(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    merge_nodes_ref(&head)
}

#[allow(dead_code)]
fn merge_nodes_ref(head: &Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    match head {
        //// 如果當前節點為None表示串列結束，回傳None
        None => None,
        Some(node) => {
            //// 如果當前節點為0，遞迴處裡下一個節點
            if node.val == 0 {
                merge_nodes_ref(&node.next)
            } else {
                //// 當前節點不為零則必有下一個節點
                //// 保存下一個節點的遞迴處理結果
                let next_node = merge_nodes_ref(&node.next);
                //// 如果下一個節點為0表示當前節點為區間內最後一個節點，直接回傳結果
                if node.next.as_ref().unwrap().val == 0 {
                    Some(Box::new(ListNode {
                        val: node.val,
                        next: next_node,
                    }))
                //// 如果下一個節點不為0，則與前面保存遞迴結果的第一項相加得到新的節點
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
        let input = ListNode::create_linked_list(vec![0, 3, 1, 0, 4, 5, 2, 0]);
        let expected_output = ListNode::create_linked_list(vec![4, 11]);

        assert_eq!(merge_nodes(input), expected_output);
    }

    #[test]
    fn test_merge_nodes_2() {
        let input = ListNode::create_linked_list(vec![0, 1, 0, 3, 0, 2, 2, 0]);
        let expected_output = ListNode::create_linked_list(vec![1, 3, 4]);

        assert_eq!(merge_nodes(input), expected_output);
    }
}
