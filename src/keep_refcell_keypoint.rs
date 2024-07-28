use std::cell::RefCell;

struct Gfx {
    config: RefCell<Config>,
}
struct Config {
    width: u32,
    height: u32,
}

pub fn run() {
    let gfx = Gfx {
        config: RefCell::new(Config {
            width: 800,
            height: 600,
        }),
    };
    println!("before: {:?}", gfx.config.borrow().width);
    just_ref_gfx(&gfx);
    println!("after: {:?}", gfx.config.borrow().width);
}

fn just_ref_gfx(gfx: &Gfx) {
    gfx.config.borrow_mut().width = 12
}

// 记得主要争对的是结构体的引用情况下可以更改的情况