use std::path::Components;

trait Comp {
    fn draw(&self);
}

struct Button {
    label: String,
    click: Box<dyn Fn() -> String>,
}

impl Comp for Button {
    fn draw(&self) {
        println!(
            "[A BUTTON] : {} and run click Fn result: {} ",
            self.label,
            (self.click)()
        );
    }
}

struct Text {
    label: String,
}
impl Comp for Text {
    fn draw(&self) {
        println!("[A TEXT] : {} ", self.label);
    }
}

struct GUI {
    comps: Vec<Box<dyn Comp>>,
}
impl GUI {
    fn new() -> GUI {
        GUI { comps: Vec::new() }
    }

    fn add_comp(&mut self, comp: Box<dyn Comp>) {
        self.comps.push(comp);
    }
    fn draw(&self) {
        for comp in &self.comps {
            comp.draw()
        }
    }
}

pub fn run() {
    let mut gui = GUI::new();
    gui.add_comp(Box::new(Button {
        label: "OK".to_string(),
        click: Box::new(|| "clicked".to_string()),
    }));
    gui.add_comp(Box::new(Text {
        label: "introduction".to_string(),
    }));
    gui.draw();
}
