pub fn run() {
    fn entry(fn_param: fn()) {
        fn_param();
    }

    fn print_it() {
        println!("print_it");
    }

    entry(print_it);

    let here_str = "only at here str";
    Wrapper { str: here_str }.print_it();
}
struct Wrapper<'a> {
    str: &'a str,
}
impl Wrapper<'_> {
    fn print_it(&self) {
        println!("{}", self.str);
    }
}