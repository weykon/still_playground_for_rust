#[derive(Debug)]
struct  First<'first> { 
    next: &'first Second
}

#[derive(Debug)]
struct Second;


pub fn run () { 
    // 看看谁先用；

    let second = Second;
    let first = First { 
        next: &second
    };
    println!("first: {:?}", first);

    // 生命周期的执行顺序类似洋葱模型
}