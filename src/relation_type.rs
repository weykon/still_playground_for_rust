struct Ground {
    characters: Vec<Box<dyn Character>>,
}
trait Character {
    fn walk(&self);
}

struct Person {
    name: String,
}
struct Animal {
    name: String,
}

trait Animation {
    fn play(&self);
}

struct WalkAnima {}


pub fn run() {
    
}