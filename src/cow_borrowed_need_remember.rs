// 过程比喻： 图书馆

use std::borrow::{BorrowMut, Cow};

#[derive(Clone)]
struct Book {
    name: String,
    content: String,
}

pub fn run() {
    let book = Book{
        name: "Rust".to_string(),
        content: "Rust is a systems programming language that runs blazingly fast, prevents segfaults, and guarantees thread safety.".to_string(),
    };

    let borrowed_book = Cow::Borrowed(&book);

    // 然后第二天想借一本可以修改的书，借之前，直接拷贝一个副本给你，随便你怎么改，不影响我
    let mut can_modify_book_from_borrowed = borrowed_book.into_owned();
    can_modify_book_from_borrowed
        .content
        .push_str(" And it is easy to learn.");
}

// 主要是对于动态的数据中，不确定处理的逻辑是否需要克隆，所以 Cow 做到的当需要修改时，再克隆，不需要修改时，不克隆
// 比如举一个数组里面的例子

fn array_string() {
    let array_words = ["Rust", "Typescript", "C"];

    for (index, &lng) in array_words.iter().enumerate() {
        if index & 1 == 0 {
            println!("{} is a systems programming language", lng);
            let mut new_copy: Cow<str> = Cow::Owned(lng.to_owned());
            new_copy.to_mut().push_str(" is the best language");
        } else {
            let just_read = Cow::Borrowed(lng);
            println!("{} is a scripting language", just_read);
        }
    }
}
