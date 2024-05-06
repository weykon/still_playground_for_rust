struct Woman(String);
struct Man(String);
trait Human {
    fn eat(&self) {}
    fn clear(&self) {}
    fn get_name(&self) -> &str;
    fn proclaim(&self) {
        println!("proclaim: {}", self.get_name())
    }
}
struct Food;
trait Eatable<T: Human> {
    fn eating(&self, human: &T);
    fn eaten(&self, human: &T);
}

impl<T: Human> Eatable<T> for Food {
    fn eating(&self, human: &T) {
        println!("from {} : Eating food", human.get_name());
        human.eat();
        human.proclaim();
    }
    fn eaten(&self, human: &T) {
        println!("from {} : Eating food", human.get_name());
        human.clear();
        human.proclaim();
    }
}

impl Human for Woman {
    fn eat(&self) {}
    fn clear(&self) {}
    fn get_name(&self) -> &str {
        &self.0
    }
}

impl Human for Man {
    fn eat(&self) {}
    fn clear(&self) {}
    fn get_name(&self) -> &str {
        &self.0
    }
}

pub(crate) fn run() {
    let food = Food;
    let heli = Woman("woman".to_string());

    food.eating(&heli);
    food.eaten(&heli);

    let apple = Food;
    apple.eating(&heli);
    apple.eaten(&heli);
}
