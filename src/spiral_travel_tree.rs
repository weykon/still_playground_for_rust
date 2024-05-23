use std::{cell::RefCell, collections::VecDeque, ops::Deref, rc::Rc};

#[derive(Debug)]
struct TreeNode {
    val: i32,
    left: Option<Rc<RefCell<TreeNode>>>,
    right: Option<Rc<RefCell<TreeNode>>>,
}

pub fn run() {
    // 创建一个用于一个三层二叉树做螺旋遍历 1324567
    let mut root = TreeNode {
        val: 1,
        left: Some(Rc::new(RefCell::new(TreeNode {
            val: 3,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 4,
                left: None,
                right: None,
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 5,
                left: None,
                right: None,
            }))),
        }))),
        right: Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 6,
                left: None,
                right: None,
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 7,
                left: None,
                right: None,
            }))),
        }))),
    };
    println!("root: {:?}", root);
    spiral_travel(Some(Rc::new(RefCell::new(root))));
}

fn spiral_travel(tree: Option<Rc<RefCell<TreeNode>>>) {
    let mut layer_vec = VecDeque::new();
    let tree = if let Some(tree) = tree {
        tree
    } else {
        return;
    };
    layer_vec.push_back(tree);
    let mut ltr = true;
    println!("layer_vec: {:?}", layer_vec.len());
    while !layer_vec.is_empty() {
        let layer_exec_count = layer_vec.len();
        for i in 0..layer_exec_count {
            if ltr {
                if let Some(tree) = layer_vec.pop_front() {
                    println!("current walk at : {:?} ", tree.clone().borrow().val);
                    if let Some(ref l) = tree.clone().borrow().left {
                        layer_vec.push_back(l.clone());
                    }
                    if let Some(ref r) = tree.clone().borrow().right {
                        layer_vec.push_back(r.clone());
                    }
                }
            } else {
                if let Some(tree) = layer_vec.pop_back() {
                    println!("current walk at : {:?} ", tree.clone().borrow().val);
                    if let Some(ref r) = tree.clone().borrow().right {
                        layer_vec.push_front(r.clone());
                    }
                    if let Some(ref l) = tree.clone().borrow().left {
                        layer_vec.push_front(l.clone());
                    }
                }
            }
        }
        ltr = !ltr;
    }
}
