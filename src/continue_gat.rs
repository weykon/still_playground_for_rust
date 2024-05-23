use std::{cell::RefCell, rc::Rc};

pub fn run() {
    println!("Continue GAT run!");

    let button = Button {
        content: "new button".to_string(),
    };
    let mut canvas2 = Canvas2D {
        draw_data: Rc::new(RefCell::new(String::new())),
        objs: vec![],
    };

    canvas2.add_widget(Box::new(button));

    canvas2.draw();
}

trait Widget {
    type CanvasAt;
    fn draw(&self, canvas: &Self::CanvasAt);
    fn type_name(&self) -> &'static str;
}

struct Canvas2D {
    draw_data: Rc<RefCell<String>>,
    objs: Vec<Box<dyn Widget<CanvasAt = Canvas2D>>>,
}
struct Canvas3D;

struct Button {
    content: String,
}
impl Widget for Button {
    type CanvasAt = Canvas2D;
    fn draw(&self, canvas: &Canvas2D) {
        canvas
            .draw_data
            .borrow_mut()
            .push_str(&self.content.to_owned());
        println!("draw button as: {}", &self.content);
    }
    fn type_name(&self) -> &'static str {
        "Button"
    }
}

struct Model3D;
impl Widget for Model3D {
    type CanvasAt = Canvas3D;
    fn draw(&self, canvas: &Canvas3D) {
        canvas;
    }
    fn type_name(&self) -> &'static str {
        "Model3D"
    }
}

impl Canvas2D {
    fn add_widget(&mut self, widget: Box<dyn Widget<CanvasAt = Self>>) {
        let widget_name = widget.type_name();
        let _ = &self.objs.push(widget);
        println!("add widget {}", widget_name);
    }
    fn draw(&mut self) {
        let objs = &self.objs;
        for widget in objs {
            widget.draw(self);
        }
    }
}
