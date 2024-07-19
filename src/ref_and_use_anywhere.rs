struct App {
    sub1: Sub1_App,
    data: i32,
}

struct Sub1_App {}
impl Sub1_App {
    fn process_app_data(&self, data: &i32) {
        println!("process_app_data: {}", data);
    }
}
pub fn run() {
    {
        // 'a
        let app = App {
            sub1: Sub1_App {},
            data: 0,
        };

        {
            // 'b
            let sub1 = &app.sub1;
            sub1.process_app_data(&app.data);
        }
    }
}
