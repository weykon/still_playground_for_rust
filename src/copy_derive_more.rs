pub fn run() {
    // 需要解说的是，关于换位复制，
    // 由于有换位复制的功能，
    // 对值与位置的关系，有了更多的可能性；
    // 所以要拥有换位复制，
    // 要确保关于引用的时候，位置的数据情况

    let just_num = 1;
    let to_new_place = just_num;
    // 关于i32本身，在rust这个类型已经实现了Copy特性，
    // 也就是平时使用时，赋值时会自动复制。
    // 这里内存中有两个为1的i32,都有各自的所有权，
    // 不是换位复制。
    // 在Rust里，换位复制是换了位置，原来的位置丧失了所有权
    println!("{} {} ", just_num, to_new_place);

    // 但是关于复合类型
    let just_str = String::from("hello");
    // String是堆上的数据
    let to_new_place = just_str;
    // Rust里当简单如此一个变量赋值另一个变量，
    // 默认执行的是换位复制，也就是原来的数据不变，也不需要复制什么
    // 只有把原来的数据地址给了一个新的变量去使用。
    // 所以对于没有实现Copy的String堆变量来讲。
    // 这里就是将just_str的所有权转移到to_new_place
    // println!("just_str: {:?}", just_str);
    // 所以上面这条会在just_str报错

    // 现在声明一个带有Copy的struct
    let can_move = CanMove { x: 1, y: 2 };
    let to_new_place = can_move;
    println!("can_move: {:?}", can_move);
    println!("to_new_place: {:?}", to_new_place);
}

#[derive(Copy, Clone, Debug)] //Clone也在因为Copy: Clone
struct CanMove {
    x: i32,
    y: i32,
}

fn move_or_copy(s: String, n: i32) -> (String, i32) {
    let s = s.clone();
    let n = n;
    (s, n)
}
// 以上对于函数，传入的参数，比如String，未Copy的类型，
// 是移动语义（Move语义），所以传入的内容的本身具备什么语义下
// 到里面就执行什么语义。
// 还需要理解，词法作用域。
// let的时候默认创建的词法作用域
// 还有{},函数，match, for等其他循环，流程控制，

fn what_borrow(mut v: [i32; 3]) -> [i32; 3] {
    v[0] = 1;
    v
}
// 借用例子，这里的 mut v: 是最简单的模式，标识符模式
// 所以如果在使用 foo(v) 的时候，也会把 (v) -> mut v 的操作
// what_borrow( Mut(v) : [i32;3]  )

// 借用默写：
// 1. 生命周期，比较好理解
// 2. 可变借用不能有alias，独占性。
// 3. 不可变借用不能再次出借给可变引用。

