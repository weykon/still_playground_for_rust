use core::any::Any;

struct Scene1 {
    name: String,
}
struct Scene2 {
    name: String,
}
trait Scene {
    fn run(&self) {
        println!("Scene::run()");
    }
}
impl Scene for Scene2 {
    fn run(&self) {
        println!("Scene2::run()");
    }
}
impl Scene for Scene1 {
    fn run(&self) {
        println!("Scene1::run()");
    }
}
impl CanOnResize for Scene1 {
    fn on_resize(&self, width: u32, height: u32) {
        println!("Scene1::on_resize({}, {})", width, height);
    }
}
trait CanOnResize {
    fn on_resize(&self, width: u32, height: u32);
}

// 当如果直接使用impl Any 给 dyn Scene 的话，就要需要每个Scene添加对于as_any的函数。
// 那么在这个点上，再加一层trait

trait AsAny {
    fn as_any(&self) -> &dyn Any;
}
impl<T: Any + Scene> AsAny for T {
    fn as_any(&self) -> &dyn Any {
        self
    }
}
trait AnyScene: Scene + AsAny {}
impl<T: AsAny + Scene> AnyScene for T {}

pub fn run() {
    let scene1 = Scene1 {
        name: "scene1".to_string(),
    };
    let scene2 = Scene2 {
        name: "scene2".to_string(),
    };

    let mut vec: Vec<Box<dyn AnyScene>> = Vec::<Box<dyn AnyScene>>::new();
    vec.push(Box::new(scene1));
    vec.push(Box::new(scene2));
    vec.iter().for_each(|scene| {
        scene.run();
        // 没有办法，只能到struct，对于trait对象不可以
        // 或许可以收集一个
        if let Some(scene) = (*scene).as_any().downcast_ref::<Scene1>() {
            scene.on_resize(32u32, 32u32);
        }
    })
}
