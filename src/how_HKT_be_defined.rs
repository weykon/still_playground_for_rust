use std::{fmt::Pointer, rc::Rc, sync::Arc};
mod step1 {
    use std::rc::Rc;

    struct Foo {
        bar: Rc<String>,
    }
    // 为了使其线程安全，您可以将 Rc 更改为 Arc ，
    // 但在某些情况下这可能会更慢。你想同时提供两者；
    // 您可以手动编写两种类型 FooRc 和 FooArc 但这会导致重复的代码。
    // 您想要抽象指针类型*：
}

// struct Foo<P: Pointer> {
//     bar: P<String>,
// }
// 但这不能编译。您想要 P<T> ，其中 P 是通用的
// （有时称为更高种类类型，HKT）。
// Rust 没有这些，但是使用 GAT 你可以模拟它们：

// code simplified for example,
// you probably need add some trait bounds and
// other functions in real code
trait PointerFamily {
    type Pointer<T>;
}

struct ArcFamily; // Just a marker type; could also use e.g. an empty enum
impl PointerFamily for ArcFamily {
    type Pointer<T> = Arc<T>;
}
// 现在 PointerFamily::Pointer<T> 让你对 PointerFamily 变得通用：

struct RcFamily;
impl PointerFamily for RcFamily {
    type Pointer<T> = Rc<T>;
}
struct Foo<P: PointerFamily> {
    bar: P::Pointer<String>,
}
// 您甚至可以提供类型别名以方便用户使用：

type FooRc = Foo<RcFamily>;
type FooArc = Foo<ArcFamily>;
// 从技术上讲，您可以在此处使用宏来避免重复，但这会更难编写，并且对用户造成限制。

mod me {
    use std::{collections::HashMap, fmt::Debug};

    pub trait Container {
        type Item<T>;
        fn run_fast_by_itself<T: Debug>(&self, item: Self::Item<T>) {
            println!("run_fast_by_itself");
        }
    }
    #[derive(Debug)]
    pub struct ArrContainer;
    impl Container for ArrContainer {
        type Item<T> = Vec<T>;
        fn run_fast_by_itself<T: Debug>(&self, item: Self::Item<T>) {
            println!("run_fast_by_itself");
            item.iter().for_each(|x| {
                println!("{:?}", x);
            })
        }
    }
    #[derive(Debug)]
    pub struct MapContainer;
    impl Container for MapContainer {
        type Item<T> = HashMap<String, T>;
    }
    #[derive(Debug)]
    pub struct TreeContainer;
    impl Container for TreeContainer {
        type Item<T> = HashMap<String, T>;
    }

    #[derive(Debug)]
    pub struct ContainerCommonData<C: Container> {
        is: String,
        container: C,
    }
    pub fn run() {
        let arr = ContainerCommonData::<ArrContainer> {
            is: "arr".to_string(),
            container: ArrContainer,
        };
        let map = ContainerCommonData::<MapContainer> {
            is: "map".to_string(),
            container: MapContainer,
        };
        let tree = ContainerCommonData::<TreeContainer> {
            is: "tree".to_string(),
            container: TreeContainer,
        };
        println!("{:?}", arr.is);
        arr.container.run_fast_by_itself(vec![1, 2, 3]);
        println!("{:?}", map.is);
        println!("{:?}", tree.is);
    }
}
pub fn run() {
    me::run();
}
