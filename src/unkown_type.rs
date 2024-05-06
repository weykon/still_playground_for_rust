struct Woman;
struct Man;
trait Human {
    fn eat(&self) {}
    fn clear(&self) {}
}
struct Food;
trait Eatable<T: Human> {
    fn eating(&self, human: &T);
    fn eaten(&self, human: &T);
}

impl<T: Human> Eatable<T> for Food {
    fn eating(&self, human: &T) {
        println!("Eating food");
        human.eat();
    }
    fn eaten(&self, human: &T) {
        println!("Eating food");
        human.clear();
    }
}

impl Human for Woman {
    fn eat(&self, human: &T) {}
    fn clear(&self, human: &T) {}
}

impl Human for Man {
    fn eat(&self, human: &T) {}
    fn clear(&self, human: &T) {}
}

fn run() {
    let food = Food;
    let heli = Woman;

    food.eating(&heli);
    food.eaten(&heli);

    let apple = Food;
    apple.eating(&heli);
    apple.eaten(&heli);

    
}
