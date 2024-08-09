pub fn run() {
    main1();
    main2();
}

use std::{borrow::BorrowMut, marker::PhantomPinned, pin::Pin};

struct MyStruct {
    data: String,
}

impl MyStruct {
    fn get_data(self: Pin<&Self>) -> &str {
        &self.get_ref().data
    }
}

fn main1() {
    let my_struct = MyStruct {
        data: "Hello".to_string(),
    };
    let pinned = Pin::new(&my_struct);
    println!("{}", pinned.get_data());
}

// 自引用结构下
struct SelfReferential {
    value: String,
    pointer_to_value: *const String,
    // Phantom: 幽灵
    // 当一个结构体包含PhantomPinned字段时,该结构体就不会自动实现Unpin trait。
    // 也就是当前的SelfReferential不会实现Unpin
    // Unpin是可以放弃安全地被移动的特性，也就是允许不安全
    _pin: PhantomPinned,
}

//----------------- 解说一下Box可以被移动的例子 -----------------//
struct Boxes {
    data: Box<i32>,
}
struct Boxes2 {
    new_data: Box<i32>,
}
fn move_box(data: Box<i32>) -> Boxes2 {
    Boxes2 { new_data: data }
}
fn test_box() {
    let data = Box::new(10);
    let new_data = move_box(data);

    println!("on new {}", new_data.new_data);
    // println!("on old {}", data);  // 这条已经报错，这个data已经被移动了
}
//----------------- 解说一下Box可以被移动的例子 -----------------//

struct NotUnpin {
    data: u32,
    _pin: PhantomPinned,
}

fn main2() {
    let at_not_unpin_struct = NotUnpin {
        data: 10,
        _pin: PhantomPinned,
    };
    let mut pinned = Box::pin(at_not_unpin_struct);
    println!("print data {}", pinned.data);

    println!("then I want to change data value: ");
    unsafe {
        let can_mut_ptr_for_struct = Pin::as_mut(&mut pinned);
        let mut_pinned = Pin::get_unchecked_mut(can_mut_ptr_for_struct);
        mut_pinned.data = 100;
    }
    println!("print data {}", pinned.data);
}
// 使用Box::pin 可以将这个struct存入堆中，然后将它pin在一个内存的固定位置。



// pin 数据的场景需要
// Pinning an object in Rust ensures that the object remains in a fixed location in memory (vital for asynchronous programming).
// 
// Pinning can help prevent data races and other concurrency issues from arising when multiple processes access the same data.
// 
// Pinning can also help improve performance by reducing the copying and moving of the code when working with asynchronous data.
// 
// Pinning ensures that certain types of data are always available in memory, even if the computer swaps out other parts of the program.