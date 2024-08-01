use std::any::Any; // 是core里面的any是同一个

// Animal
trait Animal {
    fn talk(&self);
}

// 通用寄望析构出any
trait AsAny {
    fn as_any(&self) -> &dyn Any;
}
// 进而进行和animal与any的绑定在AsAny上
impl<T: Any + Animal> AsAny for T {
    fn as_any(&self) -> &dyn Any {
        self
    }
}
// 白话: 任何实现了Animal的类型，都可以通过as_any转换成Any

// Any 内部主要就是type_id, 具体类型的标识，从而在动态上运行时进行类型的判断

struct Dog;
impl Dog {
    fn bark(&self) {
        println!("Dog::bark()");
    }
}
impl Animal for Dog {
    fn talk(&self) {
        println!("Woof!");
    }
}

pub fn run() {
    let dog = Dog;
    let as_any = dog.as_any();

    dog.talk();
    if let Some(dog) = as_any.downcast_ref::<Dog>() {
        dog.bark();
    } else {
        println!("Not a dog");
    }

}

mod if_sandy {
    use std::{any::Any, sync::Arc};

    struct Gfx {
        window: Arc<i32>,
    }
    type Window = Arc<i32>;
    trait Sandy: AsAny {
        type Extra: Any;
        fn ready(gfx: &Gfx, extra: Self::Extra);
    }
    trait AsAny {
        fn as_any(&self) -> &dyn Any;
    }
    trait Painter: Any {
        fn paint(&self);
    }
    trait Scene {
        fn render(&self);
    }
    struct Scene1 {
        need_window: Box<Window>,
    }
    impl<T: Sandy + Painter + Scene + Any> AsAny for T {
        fn as_any(&self) -> &dyn Any {
            self
        }
    }
    impl Sandy for Scene1 {
        type Extra = Window;
        fn ready(gfx: &Gfx, window_ref: Window) {
            println!("Scene1::ready()\n need_window: {}", *window_ref);
        }
    }
    impl Painter for Scene1 {
        fn paint(&self) {
            println!("Scene1::paint()");
        }
    }
    impl Scene for Scene1 {
        fn render(&self) {
            println!("Scene1::render()");
            self.paint();
        }
    }

    struct Studio {
        gfx: Gfx,
        scenes: Vec<Box<dyn Scene>>,
    }
    impl Studio {
        fn new() -> Self {
            Self {
                gfx: Gfx {
                    window: Arc::new(0),
                },
                scenes: Vec::new(),
            }
        }
        fn add_scene<T>(&mut self, extra: T::Extra)
        where
            T: Sandy + Painter + Any,
        {
            T::ready(&self.gfx, extra)
        }
        fn render_scene<T>(&self, scene: T)
        where
            T: Scene,
        {
            scene.render();
        }
    }
}
