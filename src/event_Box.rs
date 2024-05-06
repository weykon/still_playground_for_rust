use crate::Module;

struct EventManager {
    events: Vec<Box<dyn Event>>,
}

trait Event {
    fn handle(&self);
}

struct LoginEvent {
    user_id: i32,
}

impl Event for LoginEvent {
    fn handle(&self) {
        println!("LoginEvent: {}", self.user_id);
    }
}

impl EventManager {
    fn new() -> EventManager {
        EventManager { events: Vec::new() }
    }

    fn add_event(&mut self, event: Box<dyn Event>) {
        self.events.push(event);
    }

    fn process_events(&self) {
        for event in &self.events {
            event.handle();
        }
    }
}

pub fn run () {
    
}