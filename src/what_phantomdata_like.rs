use std::{fmt::Debug, marker::PhantomData};

struct MyStruct<T> {
    data: T,
    _marker: PhantomData<T>,
}
impl<T> MyStruct<T> {
    fn new(data: T) -> Self {
        MyStruct {
            data,
            _marker: PhantomData,
        }
    }
}

pub fn run() {
    println!("Hello, PhantomData!");
    let my_struct = MyStruct::new("Hello");
    println!("data: {}", my_struct.data);
    // 在这个例子中，MyStruct 结构体包含一个 PhantomData<T> 字段，
    // 用于标记 T 类型。虽然 MyStruct 实例中没有实际存储 T 类型的值，
    // 但编译器知道 MyStruct 与 T 类型相关联。

    let array = vec![1, 2, 3, 4];
    let slice = Slice::new(array.as_ptr(), unsafe { array.as_ptr().add(array.len()) });
    // slice 的生命周期与 array 一致

    // 使用引用
    let x = 42;
    let y = &x;
    println!("y: {}", y);

    // 使用 Box
    let boxed = Box::new(42);
    println!("boxed: {}", boxed);

    // 使用原生指针
    let array = vec![1, 2, 3, 4];
    let slice = Slice::new(array.as_ptr(), unsafe { array.as_ptr().add(array.len()) });
    println!("slice start: {:?}", slice.start);
    println!("slice end: {:?}", slice.end);

    slice.into_ref();
}

struct Slice<'a, T> {
    start: *const T,
    end: *const T,
    __: PhantomData<&'a T>,
}
impl<'a, T: Debug> Slice<'a, T> {
    fn new(start: *const T, end: *const T) -> Self {
        Slice {
            start,
            end,
            __: PhantomData,
        }
    }

    fn into_ref(&self) {
        unsafe {
            let mut current = self.start;
            print!("the slice contained:[ ");
            while current != self.end {
                print!("{:?} ", *current);
                current = current.add(1);
            }
            println!("]");
        }
    }
}
