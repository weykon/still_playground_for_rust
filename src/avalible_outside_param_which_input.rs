fn take_a_outside_param_not_sure_life<'a>(input: &'a i32) {
    println!("input: {}", input);
}

pub fn run() {
    let a = 10;
    take_a_outside_param_not_sure_life(&a);
    println!("a: {}", a);

    let surface = 10;
    let app = App {
        surface: &surface,
        normal: 20,
    };
}

struct App<'a> {
    surface: &'a i32, // 与从app声明的地方就同在的生命周期（至少比'a一样长）
    normal: i32,
}
