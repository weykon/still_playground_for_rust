struct App {
    start_entry: Box<dyn Scene>,
    start_entry_with_enum: EntryType,
}

trait Scene {
    fn run(&self) {
        println!("Scene::run()");
    }
}

trait NeedLoad {
    fn load(&self) {
        println!("NeedLoad::load()");
    }
}

// 可以使用Enum
enum EntryType {
    Scene(Box<dyn Scene>),
    NeedLoad(Box<dyn NeedLoad>),
}

pub fn run() {
    let app = App {
        start_entry: Box::new(Scene1 {
            name: "Scene1".to_string(),
        }),
        start_entry_with_enum: EntryType::NeedLoad(Box::new(Scene2 {
            name: "Scene2".to_string(),
        })),
    };

    app.start_entry.run();
    try_load_if_exist(app.start_entry_with_enum);

    main();
}

struct Scene1 {
    name: String,
}
impl Scene for Scene1 {
    fn run(&self) {
        println!("Scene1::run()");
    }
}
struct Scene2 {
    name: String,
}
impl Scene for Scene2 {
    fn run(&self) {
        println!("Scene2::run()");
    }
}
impl NeedLoad for Scene2 {
    fn load(&self) {
        println!("Scene2::load()");
    }
}

fn try_load_if_exist(scene: EntryType) {
    match scene {
        EntryType::Scene(ref scene) => scene.run(),
        EntryType::NeedLoad(ref need_load) => need_load.load(),
    }
}

use core::any::Any;

pub trait Animal {
    fn talk(&self);
}

struct Cat {}
struct Dog {
    pub name: String,
}

impl Animal for Cat {
    fn talk(&self) {
        println!("Meow!");
    }
}
impl Animal for Dog {
    fn talk(&self) {
        println!("Woof!");
    }
}
trait AsAny {
    fn as_any(&self) -> &dyn Any;
}
impl<T: Any + Animal> AsAny for T {
    fn as_any(&self) -> &dyn Any {
        self
    }
}
trait AnyAnimal: Animal + AsAny {}
impl<T: Animal + AsAny> AnyAnimal for T {}
type BoxedAnimal = Box<dyn AnyAnimal>;

fn main() {
    let c = Cat {};
    let d = Dog {
        name: "Fido".to_string(),
    };

    let mut zoo = Vec::<BoxedAnimal>::new();
    zoo.push(Box::new(c));
    zoo.push(Box::new(d));

    for a in zoo.iter() {
        a.talk(); //this works as expected
    }

    // How do I now get the dog's name?
    let x = &zoo[1];
    let dog = x.as_any().downcast_ref::<Dog>().unwrap();
    println!("{}", dog.name);
}
