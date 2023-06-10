use std::cell::RefCell;
use std::collections::HashMap;
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_bst1() {
        let input = vec![
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
        ];
        let root = TreeNode::create_bst(&input);
        assert_eq!(deepest_leaves_sum(root), 15);
    }

    #[test]
    fn test_create_bst2() {
        let input = vec![
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
        ];
        let root = TreeNode::create_bst(&input);
        assert_eq!(deepest_leaves_sum(root), 19);
    }
}
