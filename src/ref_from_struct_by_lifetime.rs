struct Author {
    name: String,
    birth_year: i32,
}

struct Book<'a> {
    title: String,
    author: &'a Author,
}

struct Printer {
    name: String,
}
struct Paper {
    title: String,
}
impl Printer {
    fn print(&self, book: &Book) {
        println!(
            "printer printed: {} was written by {}",
            book.title, book.author.name
        );
    }
    fn print_paper(&self, paper: &Paper) {
        println!("printer printed: {}", paper.title);
    }
}

pub fn run() {
    let author = Author {
        name: "George Orwell".to_string(),
        birth_year: 1903,
    };

    let book = Book {
        title: "1984".to_string(),
        author: &author,
    };

    println!("{} was written by {}", book.title, book.author.name);

    let printer = Printer {
        name: "Canno Printer".to_string(),
    };
    printer.print(&book);

    let paper = Paper {
        title: "A4".to_string(),
    };
    printer.print_paper(&paper);
}

// 之前上面的是在函数参数中对类型的引用，
// 函数的参数传入在写在的作用域中编译器去检查
// 但是如果是在结构体中，定义结构体的时候
// 是不知道用户会在哪里定义的，一旦实例的属性脱离了该有的周期，就会垂悬引用
struct Data {
    value: String,
}

// 所以定义同一个生命周期，属下的属性跟着实例一样同一个的周期
struct Container<'a> {
    data: &'a Data,
}

impl<'a> Container<'a> {
    fn new(data: &'a Data) -> Container<'a> {
        Container { data }
    }

    fn print(&self) {
        println!("{}", self.data.value);
    }
}
