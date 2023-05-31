use std::cell::RefCell;
use std::rc::Rc;

use crate::data_struct::binary_tree_node::*;

pub fn balance_bst(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
    let temp = inorder_traversal(&root);

    create_bst(&temp, 0, temp.len() as i32 - 1)
}

fn inorder_traversal(node: &Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    let mut temp: Vec<i32> = Vec::new();

    if let Some(n) = node {
        if let Some(l) = &n.borrow().left {
            temp.extend(inorder_traversal(&n.borrow().left));
        }

        temp.push(n.borrow().val);

        if let Some(r) = &n.borrow().right {
            temp.extend(inorder_traversal(&n.borrow().right));
        }
    }

    temp
}

//// 由於輸入的陣列為有續排列，可以用bottom-up方式建構balance binary search tree
//// 原理將中間值取出後，分成較大較小兩堆再生成子樹
//// 透過這個方式可以減少旋轉的次數
fn create_bst(source: &Vec<i32>, start: i32, end: i32) -> Option<Rc<RefCell<TreeNode>>> {
    if start > end {
        return None;
    }

    let mid = start + (end - start) / 2;

    let left = create_bst(source, start, mid - 1);
    let right = create_bst(source, mid + 1, end);

    Some(Rc::new(RefCell::new(TreeNode {
        val: source[mid as usize],
        left,
        right,
    })))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn test_1() {
        let input =
            TreeNode::create_bst(&vec![Some(1), None, Some(2), None, Some(3), None, Some(4)]);

        let expect =
            TreeNode::create_bst(&vec![Some(2), Some(1), Some(3), None, None, None, Some(4)]);
        assert_eq!(balance_bst(input), expect)
    }

    #[test]
    pub fn test_2() {
        let input = TreeNode::create_bst(&vec![Some(2), Some(1), Some(3)]);
        let expect = TreeNode::create_bst(&vec![Some(2), Some(1), Some(3)]);
        assert_eq!(balance_bst(input), expect)
    }
}
