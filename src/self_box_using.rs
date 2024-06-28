struct App {
    scene_fn: Box<dyn Fn(i32, i32)>,
    son: Son,
}
struct Son {
    data: i32,
}
pub fn run() {
    let app = App {
        son: Son { data: 1 },
        scene_fn: Box::new(|x, y| {
            println!("x: {}, y: {}", x, y);
        }),
    };
    (app.scene_fn)(1, 2);

    app.son.recall_App(&app);
}

impl Son {
    fn recall_App(&self, app: &App) {
        (app.scene_fn)(self.data, 4);
    }
}
