use core::fmt;
use std::{
    borrow::{Borrow, BorrowMut},
    fmt::{Debug, Display},
    rc::Rc,
};

trait Storable: fmt::Display + Debug {
    fn size(&self) -> usize;
}

#[derive(Debug)]
struct Data<T>
where
    T: fmt::Display,
{
    value: T,
}
impl<T> Storable for Data<T>
where
    T: fmt::Display + Debug,
{
    fn size(&self) -> usize {
        std::mem::size_of_val(&self.value)
    }
}

impl<T: fmt::Display> fmt::Display for Data<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}
struct Storage {
    items: Vec<Box<dyn Storable>>,
    extern_run_items: Vec<Rc<Box<dyn Storable>>>,
}

impl Storage {
    fn new() -> Storage {
        Storage {
            items: Vec::new(),
            extern_run_items: Vec::new(),
        }
    }
    fn add_item<T>(&mut self, item: T)
    where
        T: Storable + 'static, // 定义一下impl Storage的struct的东西在static就被定义好的在编译期中的确定性
    {
        println!("add_item: {:?}", item);
        self.items.push(Box::new(item));
    }

    fn add_item_with_extern_box(&mut self, item: &Rc<Box<dyn Storable>>) {
        println!("add_item_with_extern_box: {:?}", item);
        let can_take = item.clone();
        self.extern_run_items.push(can_take);
    }
}
pub fn run() {
    let mut storage = Storage::new();
    storage.add_item(Data { value: 3 as i32 });
    storage.add_item(Data { value: 2.2 as f32 });
    storage.add_item(Data {
        value: "data ok".to_string(),
    });
    storage.add_item(Data {
        value: Token {
            content: "jdbs27ydsf".to_string(),
        },
    });

    let global_token: Rc<Box<dyn Storable>> = Rc::new(Box::new(Data {
        value: Token {
            content: "sadad".to_string(),
        },
    }));

    storage.add_item_with_extern_box(&global_token);
    println!(
        "and here still can use global_token: {:?}",
        Rc::clone(&global_token)
    );
}
#[derive(Debug)]
struct Token {
    content: String,
}

impl Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Token: {}", self.content)
    }
}
