mod at_enum {
    enum SightingTelescope {
        RedPoint(RedPoint),
        ACOG(ACOG),
    }

    impl SightingTelescope {
        fn to_exchange_happen_first(&self) {
            match self {
                SightingTelescope::RedPoint(rp) => rp.to_exchange_happen_first(),
                SightingTelescope::ACOG(acog) => acog.to_exchange_happen_first(),
            }
        }
    }

    struct RedPoint;
    struct ACOG;

    impl RedPoint {
        fn to_exchange_happen_first(&self) {
            println!("RedPoint to_exchange_happen_first");
        }
    }

    impl ACOG {
        fn to_exchange_happen_first(&self) {
            println!("ACOG to_exchange_happen_first");
        }
    }

    struct M4;

    impl M4 {
        fn shoot(&self) {
            println!("M4 shoot");
        }

        fn change_sighting_telescope(&self, sighting_telescope: SightingTelescope) {
            sighting_telescope.to_exchange_happen_first();
        }
    }

    pub fn run() {
        let m4 = M4;
        let red_point = SightingTelescope::RedPoint(RedPoint);
        let acog = SightingTelescope::ACOG(ACOG);

        m4.change_sighting_telescope(red_point);
        m4.change_sighting_telescope(acog);
    }
}
pub fn run() {
    at_enum::run();
    gernaric_and_trait::run();
}
mod gernaric_and_trait {
    trait SightingTelescope {
        fn to_exchange_happen_first(&self);
    }

    struct RedPoint;
    struct ACOG;

    impl SightingTelescope for RedPoint {
        fn to_exchange_happen_first(&self) {
            println!("RedPoint to_exchange_happen_first");
        }
    }

    impl SightingTelescope for ACOG {
        fn to_exchange_happen_first(&self) {
            println!("ACOG to_exchange_happen_first");
        }
    }

    struct M4;

    trait Weapon {
        fn shoot(&self);
        fn change_sighting_telescope<T: SightingTelescope>(&self, sighting_telescope: T);
    }

    impl Weapon for M4 {
        fn shoot(&self) {
            println!("M4 shoot");
        }

        fn change_sighting_telescope<T: SightingTelescope>(&self, sighting_telescope: T) {
            sighting_telescope.to_exchange_happen_first();
        }
    }

    pub fn run() {
        let m4 = M4;
        let red_point = RedPoint;
        let acog = ACOG;

        m4.change_sighting_telescope(red_point);
        m4.change_sighting_telescope(acog);
    }
}
