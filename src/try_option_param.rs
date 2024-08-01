mod on_struct {
    pub struct A {}

    pub struct B {}

    pub struct DefaultParam {
        pub context: i32,
    }
    pub struct ExtraParam {
        pub context: i32,
        pub a: i32,
    }
    pub trait Sandy {
        type Param;
        fn ready(p: Self::Param);
    }
    impl Sandy for A {
        type Param = DefaultParam;
        fn ready(p: DefaultParam) {
            println!("A Default : {:?}", p.context);
        }
    }

    impl Sandy for B {
        type Param = ExtraParam;
        fn ready(p: ExtraParam) {
            println!("B Default : {:?}", p.context);
            println!("B Extra : {:?}", p.a);
        }
    }
}
pub fn run() {
    {
        use on_struct::*;
        let a = A {};
        let b = B {};

        A::ready(DefaultParam { context: 1 });
        B::ready(ExtraParam { context: 2, a: 3 });
    }
}

mod try_enum {

    #[derive(Default, Debug)]
    enum ParamType<T> {
        #[default]
        Default,
        Extra(T),
    }
    struct ParamParse<T>(T);
    trait Sandy<T> {
        fn ready(ParamParse(input): ParamParse<T>) {}
    }

    struct A {}

    impl Sandy<ParamType<()>> for A {
        fn ready(ParamParse(input): ParamParse<ParamType<()>>) {
            println!("A Default::: {:?}", input);
        }
    }

    struct B {}
    struct ExtraB {
        extra_data: i32,
    }
    impl Sandy<ParamType<ExtraB>> for B {
        fn ready(ParamParse(input): ParamParse<ParamType<ExtraB>>) {
            println!("B Default");
        }
    }
    pub fn run() {
        let a = A {};
        let b = B {};
        A::ready(ParamParse(ParamType::Default));
        B::ready(ParamParse(ParamType::Extra(ExtraB { extra_data: 1 })));
    }
}
mod try_into_thing {
    use std::{default, fmt::{Debug, Display}};

    enum ParamType<T = ()> {
        Default(),
        Extra(T),
    }
    impl Default for ParamType<()> {
        fn default() -> Self {
            ParamType::Default()
        }
    }

    impl<T> From<T> for ParamType<T> {
        fn from(t: T) -> Self {
            ParamType::Extra(t)
        }
    }

    fn input_param<T>(input: ParamType<T>) where T: Debug {
        match input {
            ParamType::Default() => {
                println!("Default");
            }
            ParamType::Extra(t) => {
                println!("Extra : {:?}", t);
            }
        }
    }
    fn run() {
        let a = ParamType::Default::<()>();
        let b = ParamType::Extra(1);
        input_param(a);
        input_param(b);
    }
}
