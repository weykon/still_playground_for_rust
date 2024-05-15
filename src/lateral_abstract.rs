use std::borrow::{Borrow, BorrowMut};

// 从一个抛弃原意OOP的概念，由一个平级和实则物体的概念，
// 比如我们初步，人是实体，而飞行员和巫师是职业，主要是执行不同的行为
// 如果需要存储的数据，那么可以用新的结构体来存储，比如PilotInfo
struct Human {
    name: String,
    age: u32,
    job: Box<dyn Job>,
}
trait Unemployed: Job {}
trait Job {
    fn get_job_name(&self) -> &str;
    fn get_job_title(&self) -> &str;
    fn get_job_level(&self) -> &str;
    fn do_job(&self);
}

trait Pilot: Job {
    fn take_to_fly(&self);
}

impl Job for PilotInfo {
    fn get_job_name(&self) -> &str {
        "Pilot"
    }
    fn get_job_title(&self) -> &str {
        "Pilot 3"
    }
    fn get_job_level(&self) -> &str {
        &self.level
    }
    fn do_job(&self) {
        println!("I am a pilot, and now I am flying.");
        println!("My name is {}, and I am {} years old.", self.name, self.age);
        println!("My level is {}.", self.level);
    }
}
impl Job for WizardInfo {
    fn get_job_name(&self) -> &str {
        "Wizard"
    }
    fn get_job_title(&self) -> &str {
        "Wizard 3"
    }
    fn get_job_level(&self) -> &str {
        &self.level
    }
    fn do_job(&self) {
        println!("I am a wizard, and now I am in a cave.");
        println!("My name is {}, and I am {} years old.", self.name, self.age);
        println!("My level is {}.", self.level);
    }
}
struct PilotInfo {
    name: String,
    age: u32,
    level: String,
}
struct WizardInfo {
    name: String,
    age: u32,
    level: String,
}

pub fn run() {
    let human = Human {
        name: "John".to_string(),
        age: 30,
        job: Box::new(PilotInfo {
            name: "John".to_string(),
            age: 30,
            level: "3".to_string(),
        }),
    };

    human.job.do_job();
}
