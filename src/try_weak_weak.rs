// weak在语义上的逻辑理解可以为
// 是否被照顾，是否被关心，是否被保护

// 或者在一段引用的关系中，主动和被动的语义关系

use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

#[derive(Debug)]
struct Node {
    next: Option<Rc<RefCell<Node>>>,
    head: Option<Weak<RefCell<Node>>>,
}

fn main() {
    let node1 = List {
        value: 1,
        next: None,
    };

    let node2 = List {
        value: 2,
        next: Some(Weak::new()),
    };
}

struct List<T> {
    value: T,
    next: Option<Weak<List<T>>>,
}

impl Drop for Node {
    fn drop(&mut self) {
        println!("Dropping");
    }
}

pub fn run() {
    let a = Rc::new(RefCell::new(Node {
        next: None,
        head: None,
    }));
    println!(
        "a strong count: {:?}, weak count: {:?}",
        Rc::strong_count(&a),
        Rc::weak_count(&a)
    );
    let b = Rc::new(RefCell::new(Node {
        next: Some(Rc::clone(&a)),
        head: None,
    }));
    println!(
        "a strong count: {:?}, weak count: {:?}",
        Rc::strong_count(&a),
        Rc::weak_count(&a)
    );
    println!(
        "b strong count: {:?}, weak count: {:?}",
        Rc::strong_count(&b),
        Rc::weak_count(&b)
    );
    let c = Rc::new(RefCell::new(Node {
        next: Some(Rc::clone(&b)),
        head: None,
    }));

    // Creates a reference cycle
    (*a).borrow_mut().head = Some(Rc::downgrade(&c));
    println!(
        "a strong count: {:?}, weak count: {:?}",
        Rc::strong_count(&a),
        Rc::weak_count(&a)
    );
    println!(
        "b strong count: {:?}, weak count: {:?}",
        Rc::strong_count(&b),
        Rc::weak_count(&b)
    );
    println!(
        "c strong count: {:?}, weak count: {:?}",
        Rc::strong_count(&c),
        Rc::weak_count(&c)
    );

    println!("a {:?}", &a);

    println!(":::::::::::::::");

    main();
}
