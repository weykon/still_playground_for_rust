pub fn run() {}

trait GameThing {
    fn get_thing_name(&self) -> String;
}

trait Display {
    fn display(&self);
}

trait NeedUpdate {
    fn update(&self);
}

impl<T> Display for T
where
    T: GameThing,
{
    fn display(&self) {
        println!("Display: {}", self.get_thing_name());
    }
}
