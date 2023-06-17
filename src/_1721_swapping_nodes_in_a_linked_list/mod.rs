use crate::data_struct::ListNode;

// 這邊的LinkedList是透過Box實現的，由於rust中不允許多個共享可變引用，所以無法直接透過swap的方式交換兩個node的值
// 所以這邊先將LinkedList轉換成Vec，再透過Vec的swap方法交換兩個node的值，最後再將Vec轉換回LinkedList
// 或者可以透過unsafe的方式，直接交換兩個node的值，但是這邊不建議這樣做
pub fn swap_nodes(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
    let mut current = &head;
    let mut vec = Vec::new();

    while let Some(node) = current {
        vec.push(node.val);
        current = &node.next;
    }

    let len = vec.len();
    let temp = vec[k as usize - 1];
    vec[k as usize - 1] = vec[len - k as usize];
    vec[len - k as usize] = temp;

    let mut result = ListNode::new(vec[0]);
    let mut c = &mut result;

    for i in 1..vec.len() {
        c.next = Some(Box::new(ListNode::new(vec[i])));
        c = c.next.as_mut().unwrap();
    }

    Some(Box::new(result))
}

mod test {
    use super::*;

    #[test]
    fn test_swap_nodes_1() {
        let input = ListNode::create_linked_list(vec![1, 2, 3, 4, 5]);
        let expected_output = ListNode::create_linked_list(vec![1, 4, 3, 2, 5]);

        assert_eq!(swap_nodes(input, 2), expected_output);
    }

    #[test]
    fn test_swap_nodes_2() {
        let input = ListNode::create_linked_list(vec![7, 9, 6, 6, 7, 8, 3, 0, 9, 5]);
        let expected_output = ListNode::create_linked_list(vec![7, 9, 6, 6, 8, 7, 3, 0, 9, 5]);

        assert_eq!(swap_nodes(input, 5), expected_output);
    }
}
