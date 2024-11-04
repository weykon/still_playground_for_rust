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
// 翻译：在Rust中固定对象的位置可以确保对象在内存中保持固定位置（对于异步编程至关重要）。
// Pinning can help prevent data races and other concurrency issues from arising when multiple processes access the same data.
// 翻译：当多个进程访问相同的数据时，固定可以帮助防止数据竞争和其他并发问题的发生。
// Pinning can also help improve performance by reducing the copying and moving of the code when working with asynchronous data.
// 翻译：当处理异步数据时，固定还可以通过减少代码的复制和移动来提高性能。
// Pinning ensures that certain types of data are always available in memory, even if the computer swaps out other parts of the program.
// 翻译：固定确保某些类型的数据始终在内存中可用，即使计算机交换程序的其他部分也是如此。

// 比如举一个需要使用pin相关的需要在异步下的特殊写法的例子

