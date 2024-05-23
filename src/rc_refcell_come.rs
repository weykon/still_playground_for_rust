// Rc<RefCell<T>>
// 对于Refcell的可用计数，想一个使用场景

use std::{cell::RefCell, rc::Rc};

struct Book {
    title: String,
    borrower: RefCell<Option<String>>,
}

fn check_book(book: &Rc<Book>) {
    match *book.borrower.borrow() {
        Some(ref name) => println!("{} is borrowed by {}", book.title, name),
        None => println!("{} is not borrowed", book.title),
    };
}

fn borrow_book(book: &Rc<Book>, who: String) {
    *book.borrower.borrow_mut() = Some(who.to_owned());
}
fn return_book(book : &Rc<Book>){
    *book.borrower.borrow_mut() = None;
}
pub fn run() {
    let book = Rc::new(Book {
        title: "Rust Book".to_string(),
        borrower: RefCell::new(None),
    });
    check_book(&book);
    borrow_book(&book, "ko".to_string());
    check_book(&book);

    return_book(&book);
    check_book(&book);
}
