use std::{
    fmt::{Debug, Display},
    vec,
};

struct Monitor {}
impl Display for Monitor {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "[Monitor]:")
    }
}
trait DisplayMonitor {
    fn display(&self);
}
impl<T: Display> DisplayMonitor for T {
    fn display(&self) {
        println!("\n***\n{}\n***\n", self);
    }
}
// --------------------------------------

pub fn run() {
    let monitor = Monitor {};
    monitor.display();

    let runs = vec![run2, run3, run4].into_iter();
    for fnc in runs {
        println!("\n");
        fnc();
    }
}

#[derive(Debug)]
struct Book {
    name: String,
    title: String,
    content: String,
}

#[derive(Debug)]
struct EBook<'a> {
    name: String,
    title: String,
    content: &'a str,
}

fn run2() {
    let book = Book {
        name: "book".to_string(),
        title: "Book".to_string(),
        content: "form book".to_string(),
    };
    let ebook = EBook {
        name: "ebook".to_string(),
        title: "ebook".to_string(),
        content: &book.content,
    };
    println!("ebook: {:?}", ebook);
}

struct MainData {
    data: Vec<Box<dyn DisplayMonitor>>,
    name: String,
}

fn run3() {
    let mut main_data = MainData {
        data: vec![],
        name: "MainData".to_string(),
    };

    main_data.name = String::from("new String");

    println!("先定义后赋值");
}

trait Fire {
    fn fire(&self);
}
trait Reload {
    fn reload(&self);
}
trait Anim {
    fn anim(&self);
}
struct Character;
struct Weapon;
impl Fire for Character {
    fn fire(&self) {
        println!("Character fire");
    }
}
impl Reload for Character {
    fn reload(&self) {
        println!("Character reload");
    }
}
impl Debug for Weapon {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "fmt Weapon")
    }
}
impl Debug for Character {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "fmt Character")
    }
}
impl Fire for Weapon {
    fn fire(&self) {
        println!("Weapon fire");
    }
}
impl Reload for Weapon {
    fn reload(&self) {
        println!("Weapon reload: {:?}", &self);
    }
}

fn run4() {
    let character = Character;
    let weapon = Weapon;
    character.fire();
    character.reload();
    weapon.fire();
    weapon.reload();
}