use std::{cell::RefCell, rc::Rc};

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }

    pub fn create_bst(arr: &Vec<Option<i32>>) -> Option<Rc<RefCell<TreeNode>>> {
            if let Some(&val) = arr.first() {
                if val == None {
                    return None;
                }

                let root = Rc::new(RefCell::new(TreeNode::new(val.unwrap())));

                let mut queue = std::collections::VecDeque::new();
                queue.push_back(Rc::clone(&root));

                let mut i = 1;
                while !queue.is_empty() && i < arr.len() {
                    if let Some(node) = queue.pop_front() {
                        if let Some(&left_val) = arr.get(i) {
                            if let Some(val) = left_val {
                                let left = Rc::new(RefCell::new(TreeNode::new(val)));
                                node.borrow_mut().left = Some(Rc::clone(&left));
                                queue.push_back(left);
                            }
                        }
                        i += 1;

                        if let Some(&right_val) = arr.get(i) {
                            if let Some(val) = right_val {
                                let right = Rc::new(RefCell::new(TreeNode::new(val)));
                                node.borrow_mut().right = Some(Rc::clone(&right));
                                queue.push_back(right);
                            }
                        }
                        i += 1;
                    }
                }
                Some(root)
            } else {
                None
            }
    }
}
