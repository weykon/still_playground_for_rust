pub fn run() {
    // higher_rank_trait_bound_run
}
mod preliminary {
    struct Closure<F> {
        data: (u8, u16),
        func: F,
    }

    impl<F> Closure<F>
    where
        F: Fn(&(u8, u16)) -> &u8,
    {
        fn call(&self) -> &u8 {
            (self.func)(&self.data)
        }
    }

    fn do_it(data: &(u8, u16)) -> &u8 {
        &data.0
    }
    fn main() {
        let clo = Closure {
            data: (0, 1),
            func: do_it,
        };
        println!("{}", clo.call());
    }
}

mod however {
    struct Closure<F> {
        data: (u8, u16),
        func: F,
    }

    impl<F> Closure<F>
    where
        for<'a> F: Fn(&'a (u8, u16)) -> &'a u8,
    {
        fn call(&self) -> &u8 {
            (self.func)(&self.data)
        }
    }

    fn do_it(data: &(u8, u16)) -> &u8 {
        &data.0
    }

    fn main() {
        let clo = Closure {
            data: (0, 1),
            func: do_it,
        };
        println!("{}", clo.call());
    }
}
