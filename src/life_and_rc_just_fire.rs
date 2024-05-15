// 主要是比如不使用指针类型的时候，可以通过声明
// 生命周期来弥补变量不知道在什么作用域的尴尬

struct Person {
    name: String,
}
struct People<'in_main> {
    name: &'in_main str,
}

pub fn run() {
    let me = Person {
        name: String::from("me"),
    };

    // 这里定义了周期，可以知道在目前作用域下维持
    // let you = People { name: "you" };

    let you;
    {
        let name = String::from("OK");
        // you = People { name: &name }; // 取消注释的话，这个时候就会是错误的
        you = Person {
            name: name,
        };
    }

    // 在这里，name的生命周期已经结束，但是you仍然持有对它的引用，所以Rust编译器会报错。
    println!("People name: {}", you.name);
}


