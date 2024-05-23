// what Weak in rust ?

use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

struct Tree {
    root: Node,
}
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Weak<Node>>>,
}

pub fn run() {
    // 复习 rc weak refcell

    // rc 引用计数-在堆上创建一个胖指针，不复制堆数据，只是创建引用，然后加一个计数功能
    // refcell 类似于比起常规的引用，它作为了一个明确语义去告诉编译器，它在borrow的时候需要变化，
    // 为什么本来的常规引用就已经可以多处引用了，在refcell的时候还需要区分了borrow和borrow_mut呢
    // 因为既然定义了这个refcell，当需要堆包裹的数据进行引用，我们调用的borrow的方法的存在，
    // 如果没有borrow的方法的话，直接&refcell，这一步只是对目前这个refcell的本身引用，非堆数据的引用

    {
        /// 以下是固定的关系图
        use rcc::*;
        let me = Rc::new(Person("me".to_string()));
        let father = Rc::new(Person("father".to_string()));
        let mom = Rc::new(Person("mon".to_string()));

        let son_father = Relationship {
            from: me.clone(),
            to: father.clone(),
            name: "son-father".to_string(),
        };
        let father_son = Relationship {
            from: father.clone(),
            to: me.clone(),
            name: "father->son".to_string(),
        };
        let mom_son = Relationship {
            from: mom.clone(),
            to: me.clone(),
            name: "mom->son".to_string(),
        };

        println!("count reference of me: {}", Rc::strong_count(&me));
        println!("count reference of father: {}", Rc::strong_count(&father));
        println!("count reference of mom: {}", Rc::strong_count(&mom));

        fn create_relationship() -> Relationship {
            let me = Rc::new(Person("me".to_string()));
            let father = Rc::new(Person("father".to_string()));

            let son_father = Relationship {
                from: me.clone(),
                to: father.clone(),
                name: "son-father".to_string(),
            };

            son_father
        }

        fn print_relationship(relationship: &Relationship) {
            println!(
                "Relationship: {} -> {}",
                (relationship.from.0),
                (relationship.to.0)
            );
        }

        fn main() {
            let relationship = create_relationship();
            print_relationship(&relationship);
        }
    }

    {
        /// 以下是常规的引用的时候的局限
        use normal::*;
        let me = Person("me".to_string());
        let father = Person("father".to_string());
        let mom = Person("mon".to_string());

        let son_father = Relationship {
            from: &me,
            to: &father,
            name: "son-father".to_string(),
        };
        let father_son = Relationship {
            from: &father,
            to: &me,
            name: "father->son".to_string(),
        };
        let mom_son = Relationship {
            from: &mom,
            to: &me,
            name: "mom->son".to_string(),
        };

        // fn create_relationship() -> Relationship {
        //     let me = Person("me".to_string());
        //     let father = Person("father".to_string());

        //     let son_father = Relationship {
        //         from: &me,
        //         to: &father,
        //         name: "son-father".to_string(),
        //     };

        //     son_father
        // }
        // fn print_relationship(relationship: &Relationship) {
        //     println!(
        //         "Relationship: {} -> {}",
        //         (relationship.from.0),
        //         (relationship.to.0)
        //     );
        // }
        // fn main() {
        //     let relationship = create_relationship();
        //     print_relationship(&relationship);
        // }
        // 以上是用了生命周期，第一很麻烦，第二无法实现
        // 从create_relationship里面创建出来的relationship后，
        // 里面的me和father已经不在了，但是print_relationship里面还打印
        // 失败的
    }

    {
        use refcellne::*;
        let man = Person("john".to_string());
        let woman = Person("lucy".to_string());
        let other_woman = Person("lily".to_string());

        let relationship = Relationship {
            from: RefCell::new(man),
            to: RefCell::new(woman),
            name: "husband-wife".to_string(),
        };

        println!("previous relationship: {:?}" , relationship);
        relationship.to.borrow_mut().0 = other_woman.0.to_owned();
        println!("now relationship: {:?}", relationship);
    } 

    {}
}

pub mod normal {
    pub struct Person(pub String);
    pub struct Relationship<'a> {
        pub from: &'a Person,
        pub to: &'a Person,
        pub name: String,
    }
}
pub mod rcc {
    use std::rc::Rc;
    pub struct Person(pub String);
    pub struct Relationship {
        pub from: Rc<Person>,
        pub to: Rc<Person>,
        pub name: String,
    }
}

mod refcellne {
    use std::borrow::Borrow;
    use std::cell::RefCell;
    use std::fmt::Debug;
    use std::rc::Rc;

    // 离婚版本
    // 需要更改关系
    #[derive(Debug)]
    pub struct Person(pub String);
    pub struct Relationship {
        pub from: RefCell<Person>,
        pub to: RefCell<Person>,
        pub name: String,
    }
    impl Debug for Relationship {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            // "* -> *"
            write!(f, "{} -> {}", self.from.borrow().0, self.to.borrow().0)
        }
    }
}
