mod enum_dispatch {
    enum CoffeeType {
        Espresso(Espresso),
        Cappuccino(Cappuccino),
        Americano(Americano),
        Latte(Latte),
    }

    trait Coffee {
        fn price(&self) -> f32;
        fn strength(&self) -> f32;
    }
    #[derive(Debug, Default)]
    enum CupSize {
        #[default]
        Small,
        Medium,
        Large,
    }
    #[derive(Debug, Default)]
    struct Espresso {
        size: CupSize,
    }

    #[derive(Debug, Default)]
    struct Cappuccino {
        size: CupSize,
    }
    #[derive(Debug, Default)]
    struct Americano {
        size: CupSize,
    }
    #[derive(Debug, Default)]
    struct Latte {
        size: CupSize,
    }

    impl CoffeeType {
        fn just_greet(&self) {
            match self {
                CoffeeType::Espresso(e) => println!("{:?} Espresso", e.size),
                CoffeeType::Cappuccino(c) => println!("{:?} Cappuccino", c.size),
                CoffeeType::Americano(a) => println!("{:?} Americano", a.size),
                CoffeeType::Latte(l) => println!("{:?} Latte", l.size),
            };
        }
    }

    pub fn run() {
        let espresso = CoffeeType::Espresso(Espresso::default());
        let cappuccino = CoffeeType::Cappuccino(Cappuccino::default());
        let americano = CoffeeType::Americano(Americano::default());
        let latte = CoffeeType::Latte(Latte::default());
        espresso.just_greet();
        cappuccino.just_greet();
        americano.just_greet();
        latte.just_greet();
    }
}

mod trait_vs_enum {
    trait CoffeeType {
        fn just_greet(&self);
    }
    trait Coffee {
        fn price(&self) -> f32;
        fn strength(&self) -> f32;
    }
    #[derive(Debug, Default)]
    enum CupSize {
        Small,
        #[default]
        Medium,
        Large,
    }
    #[derive(Debug, Default)]
    struct Espresso {
        size: CupSize,
    }
    #[derive(Debug, Default)]
    struct Cappuccino {
        size: CupSize,
    }

    impl CoffeeType for Espresso {
        fn just_greet(&self) {
            println!("{:?} Espresso", self.size);
        }
    }
    impl CoffeeType for Cappuccino {
        fn just_greet(&self) {
            println!("{:?} Cappuccino", self.size);
        }
    }

    pub fn run() {
        let espresso = Espresso::default();
        let cappuccino = Cappuccino::default();
        espresso.just_greet();
        cappuccino.just_greet();
    }
}
pub fn run() {
    enum_dispatch::run();
    trait_vs_enum::run();
}
