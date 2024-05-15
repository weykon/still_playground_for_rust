use std::any::Any;
trait Infra: Any {
    fn run(&self);
    fn as_any(&self) -> &dyn Any;
}
impl<T: 'static + Any> Infra for T {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn run(&self) {
        println!("Infra trait");
    }
}

trait Module: Infra {
    fn try_run(&self);
}
impl Module for dyn Infra {
    fn try_run(&self) {
        println!("Module trait");
    }
}
struct Main {
    files: Vec<Box<dyn Infra>>,
}
struct TestFile {
    name: String,
}

impl Module for TestFile {
    fn try_run(&self) {
        println!("TestFile Module");
    }
}
trait AsModule {
    fn as_module(&self) -> Option<&dyn Module>;
}

pub fn run() {
    let mut main = Main { files: Vec::new() };
    main.files.push(Box::new(TestFile {
        name: "TestFile".to_string(),
    }));

    main.files.iter().for_each(|file: &Box<dyn Infra>| {
        println!("------------");
        file.run();
        file.try_run();

        println!("------------");
        println!("\n");
    });
}
