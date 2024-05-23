use std::{
    collections::VecDeque,
    fmt::{Debug, Display},
};

/// 使用Box在大项目中，数据共享常常有，大跨度的文件代码引用修改调用，
/// 使用Box的话只能在一个比较局限的生命周期里面操作，
/// 而且Box如果想修改Box包裹里面的数据的再里面细分的数据的时候，
/// 是无法修改，只能从整个可变box下操作修改，
/// 不可直接在那个具体的数据处进行修改。
/// 所以最好就是保持严格的内存管理也对抽象和语义上比较负责
///
/// 使用严格的内存管理下的编码风格，更有故事性，如果使用Box的话，
/// Box的书写局限，和隐藏业务抽象语义，可能失去大部分的故事性
pub fn run() {
    let tree = TreeNode {
        value: 1,
        left: Some(Box::new(TreeNode {
            value: 2,
            left: None,
            right: None,
        })),
        right: Some(Box::new(TreeNode {
            value: 3,
            left: None,
            right: None,
        })),
    };
    println!("pre");
    pre_order::<i32>(&Some(Box::new(tree.clone())));
    println!("\nin");
    in_order::<i32>(&Some(Box::new(tree.clone())));
    println!("\npost");
    post_order::<i32>(&Some(Box::new(tree)));

    let tree_for_level = TreeNode {
        value: 1,
        left: Some(Box::new(TreeNode {
            value: 2,
            left: Some(Box::new(TreeNode {
                value: 4,
                left: None,
                right: None,
            })),
            right: Some(Box::new(TreeNode {
                value: 5,
                left: None,
                right: None,
            })),
        })),
        right: Some(Box::new(TreeNode {
            value: 3,
            left: Some(Box::new(TreeNode {
                value: 6,
                left: None,
                right: None,
            })),
            right: Some(Box::new(TreeNode {
                value: 7,
                left: None,
                right: None,
            })),
        })),
    };

    snake_order::<i32>(&Some(Box::new(tree_for_level)));
}

// 但是但是，还是得用的

#[derive(Debug, Clone)]
struct TreeNode<T> {
    value: T,
    left: Option<Box<TreeNode<T>>>,
    right: Option<Box<TreeNode<T>>>,
}

// 我想在这里练习一下前序，中序，后序遍历

fn pre_order<T: Debug>(tree: &Option<Box<TreeNode<T>>>) {
    if let Some(tree) = tree {
        println!("{:?}", tree.value);
        pre_order(&tree.left);
        pre_order(&tree.right);
    }
}
fn in_order<T: Debug>(tree: &Option<Box<TreeNode<T>>>) {
    if let Some(tree) = tree {
        in_order(&tree.left);
        println!("{:?}", tree.value);
        in_order(&tree.right);
    }
}
fn post_order<T: Debug>(tree: &Option<Box<TreeNode<T>>>) {
    if let Some(tree) = tree {
        post_order(&tree.left);
        post_order(&tree.right);
        println!("{:?}", tree.value);
    }
}

// 先来层序，默认左到右
fn snake_order<T: Debug>(tree: &Option<Box<TreeNode<T>>>) {
    let mut que = VecDeque::new();
    if let Some(tree) = tree {
        que.push_back(tree);
    }
    while !que.is_empty() {
        if let Some(tree) = que.pop_front() {
            println!("value here: {:?}", tree.value);
            if let Some(left) = &tree.left {
                que.push_back(left);
            }
            if let Some(right) = &tree.right {
                que.push_back(right);
            }
        }
    }
}
