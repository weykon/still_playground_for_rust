use std::ops::Add;

enum Sexuel {
    Man,
    Woman,
}

struct Mom {
    sex: Sexuel,
}

struct Dad {
    sex: Sexuel,
}

trait FamilyMember {
    fn social_duty(&self);
    fn sex(&self) -> &Sexuel;
}

impl FamilyMember for Mom {
    fn social_duty(&self) {
        println!("Mom's duty");
    }

    fn sex(&self) -> &Sexuel {
        &self.sex
    }
}

impl FamilyMember for Dad {
    fn social_duty(&self) {
        println!("Dad's duty");
    }

    fn sex(&self) -> &Sexuel {
        &self.sex
    }
}

struct Child {}

impl FamilyMember for Child {
    fn social_duty(&self) {
        println!("Child's duty");
    }

    fn sex(&self) -> &Sexuel {
        // Child doesn't have a specific sex in this context
        unimplemented!()
    }
}

impl Add<Dad> for Mom {
    type Output = Child;

    fn add(self, _rhs: Dad) -> Self::Output {
        Child {}
    }
}

pub fn run() {
    let mom = Mom { sex: Sexuel::Woman };
    let dad = Dad { sex: Sexuel::Man };
    let child = mom + dad;

    child.social_duty();
}