// 编一个故事，
// 有一个父亲离婚了孩子给了他，但是后面孩子又给了母亲，
// 他们对孩子的照顾作为一个定义

use std::{
    borrow::Borrow,
    cell::RefCell,
    ops::Deref,
    rc::{Rc, Weak},
};

#[derive(Debug)]
enum Relation {
    StayWithKidAllDay(Rc<bool>),
    HimOrHerStayWithKid(Weak<bool>),
    NeedBeingWiteParent(Weak<bool>),
}

struct Perent {
    with_kid: RefCell<Relation>,
}
#[derive(Debug)]
struct Kid {
    with_parent: RefCell<Relation>,
}
pub fn run() {
    // 一开始父亲照顾孩子
    let father = Perent {
        with_kid: RefCell::new(Relation::StayWithKidAllDay(Rc::new(true))),
    };

    let kid = Kid {
        with_parent: RefCell::new(Relation::NeedBeingWiteParent(Rc::downgrade(match &*father
            .with_kid
            .borrow()
        {
            Relation::StayWithKidAllDay(r) => r,
            _ => panic!(""),
        }))),
    };

    {
        let mother = Perent {
            // 此时的母亲
            with_kid: RefCell::new(Relation::HimOrHerStayWithKid(Rc::downgrade(match &*father
                .with_kid
                .borrow()
            {
                Relation::StayWithKidAllDay(r) => &r,
                _ => panic!(""),
            }))),
        };
        // 这里母亲对with_kid的引用，如果是Rc的话
    }

    let kid_relation = kid.with_parent.borrow();

    fn check_kid_relation(relation: &Relation) {
        match relation {
            Relation::NeedBeingWiteParent(r) => {
                println!("how kid now : {:?}", r.upgrade());
                if r.upgrade() == None {
                    println!("   kid is alone, poor kid");
                } else {
                    println!("   kid is not alone, good");
                }
            }
            _ => println!("panic"),
        }
    }
    check_kid_relation(&kid_relation);
    *father.with_kid.borrow_mut() = Relation::StayWithKidAllDay(Rc::new(false));
    check_kid_relation(&kid_relation);
}
