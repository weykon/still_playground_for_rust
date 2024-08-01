use std::{ops::Deref, sync::Arc};

trait Sandy {
    type Extra;
    fn ready(gfx: &Gfx, extra: Self::Extra);
}

struct Gfx {
    window: Arc<i32>,
}
type Window = Arc<i32>;
struct Scene1 {}

impl Sandy for Scene1 {
    type Extra = ();
    fn ready(gfx: &Gfx, extra: ()) {
        println!("Scene1::ready()");
    }
}

struct Scene2 {}
impl Sandy for Scene2 {
    type Extra = Box<Window>;
    fn ready(gfx: &Gfx, extra: Box<Window>) {
        println!("Scene2::ready()");
        let extra = *extra.as_ref().deref();
    }
}

struct Studio {}
impl Studio {
    fn add_scene<T>(&mut self, extra: T::Extra)
    where
        T: Sandy,
    {
        let gfx = Gfx {
            window: Arc::new(0),
        };
        T::ready(&gfx, extra);
    }
}

pub fn run() {
    let mut studio = Studio {};
    let window = Arc::new(0);
    studio.add_scene::<Scene1>(());
    studio.add_scene::<Scene2>(Box::new(window.clone()));
}
