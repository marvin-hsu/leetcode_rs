use std::cell::RefCell;
use std::collections::{HashMap, VecDeque};
use std::rc::Rc;

use crate::data_struct::binary_tree_node::*;

pub fn deepest_leaves_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    let mut map = HashMap::new();

    travesal(&root, 0, &mut map);

    *map.iter().max_by(|a, b| a.0.cmp(b.0)).unwrap().1
}

fn travesal(node: &Option<Rc<RefCell<TreeNode>>>, layer: i32, map: &mut HashMap<i32, i32>) {
    if let Some(n) = node {
        travesal(&n.borrow().left, layer + 1, map);

        map.entry(layer)
            .and_modify(|e| *e += n.borrow().val)
            .or_insert(n.borrow().val);

        travesal(&n.borrow().right, layer + 1, map);
    }
}

pub fn deepest_leaves_sum_2(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    let mut queue = VecDeque::new();
    let mut sum = 0;

    if let Some(node) = root {
        queue.push_back(node);
    }

    while !queue.is_empty() {
        let level_size = queue.len();
        sum = 0;

        for _ in 0..level_size {
            if let Some(node) = queue.pop_front() {
                sum += node.borrow().val;

                if let Some(left) = &node.borrow().left {
                    queue.push_back(left.clone());
                }

                if let Some(right) = &node.borrow().right {
                    queue.push_back(right.clone());
                }
            }
        }
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deepest_leaves_sum() {
        let root1 = TreeNode::create_bst(&vec![
            Some(1),
            Some(2),
            Some(3),
            Some(4),
            Some(5),
            None,
            Some(6),
            Some(7),
            None,
            None,
            None,
            None,
            Some(8),
        ]);
        assert_eq!(deepest_leaves_sum(root1), 15);

        let root2 = TreeNode::create_bst(&vec![
            Some(6),
            Some(7),
            Some(8),
            Some(3),
            Some(7),
            Some(1),
            Some(3),
            Some(9),
            None,
            Some(1),
            Some(4),
            None,
            None,
            None,
            Some(5),
        ]);
        assert_eq!(deepest_leaves_sum(root2), 19);
    }

    #[test]
    fn test_deepest_leaves_sum_2() {
        let root1 = TreeNode::create_bst(&vec![
            Some(1),
            Some(2),
            Some(3),
            Some(4),
            Some(5),
            None,
            Some(6),
            Some(7),
            None,
            None,
            None,
            None,
            Some(8),
        ]);
        assert_eq!(deepest_leaves_sum_2(root1), 15);

        let root2 = TreeNode::create_bst(&vec![
            Some(6),
            Some(7),
            Some(8),
            Some(3),
            Some(7),
            Some(1),
            Some(3),
            Some(9),
            None,
            Some(1),
            Some(4),
            None,
            None,
            None,
            Some(5),
        ]);
        assert_eq!(deepest_leaves_sum_2(root2), 19);
    }
}
