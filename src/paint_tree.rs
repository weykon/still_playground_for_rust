use std::{
    borrow::Borrow,
    cell::RefCell,
    collections::VecDeque,
    fmt::{Display, Write},
    ops::Deref,
    rc::Rc,
};

#[derive(Debug, Clone)]
struct TreeNode {
    val: i32,
    left: Option<Rc<RefCell<TreeNode>>>,
    right: Option<Rc<RefCell<TreeNode>>>,
}
impl Display for TreeNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        level_travel(Some(Rc::new(RefCell::new(self.clone()))), f)
    }
}

fn max_depth(tree: Option<Rc<RefCell<TreeNode>>>) -> usize {
    match tree {
        Some(node) => {
            let node = node.deref().borrow();
            1 + std::cmp::max(max_depth(node.left.clone()), max_depth(node.right.clone()))
        }
        None => 0,
    }
}

fn level_travel<W: Write>(tree: Option<Rc<RefCell<TreeNode>>>, f: &mut W) -> std::fmt::Result {
    let max_depth = max_depth(tree.clone());
    let mut max_depth = max_depth;
    write!(f, "max_depth: {}\n", max_depth)?;
    let mut layer_vec = VecDeque::new();
    let tree = if let Some(tree) = tree {
        tree
    } else {
        return Ok(());
    };
    layer_vec.push_back(tree);
    write!(f, "layer_vec: {}\n", layer_vec.len())?;
    while !layer_vec.is_empty() {
        let layer_exec_count = layer_vec.len();
        for i in 0..layer_exec_count {
            if let Some(tree) = layer_vec.pop_front() {
                let spaces = format!("{:width$}", "", width = (max_depth as f64 * 5f64.sqrt()).floor() as usize);
                let gap = format!("{:width$}", "", width = (max_depth as f64 * 2f64.sqrt()).floor() as usize + 5);
                write!(f, "{}{}{}", spaces, tree.clone().deref().borrow().val, gap)?;
                max_depth = max_depth.saturating_sub(1);
                if let Some(ref l) = tree.clone().deref().borrow().left {
                    layer_vec.push_back(l.clone());
                }
                if let Some(ref r) = tree.clone().deref().borrow().right {
                    layer_vec.push_back(r.clone());
                }
            }
        }
        write!(f, "\n")?;
    }
    Ok(())
}

pub fn run() {
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
    println!("root: {}", root);
}
