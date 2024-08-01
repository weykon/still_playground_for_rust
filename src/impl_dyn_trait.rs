use specify::Animal;

trait Trait<'a> {
    fn run(&'a self);
}

// 这个适用于是 从 dyn 的类型
impl<'a> dyn Trait<'a> {
    pub fn run(&self) {
        println!("Trait::run");
    }
}

// 这个是从具体类型的实现
impl<T> Trait<'_> for T {
    fn run(&self) {
        println!("T::run");
    }
}

struct Struct;

mod specify {
    pub(crate) trait Animal {
        fn speak(&self);
    }

    impl dyn Animal {
        pub fn describe(&self) {
            println!("Animal::describe");
            self.speak();
        }
    }
    pub struct Dog;
    impl Animal for Dog {
        fn speak(&self) {
            println!("Woof!");
        }
    }
}

pub fn run() {
    let dog: Box<dyn Animal> = Box::new(specify::Dog);
    dog.describe();
}
