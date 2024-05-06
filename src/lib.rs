mod dimension;
mod event_box;
mod game_attack;
mod payment;
mod plugin_system;
mod print_it;
mod storage;
mod ui;
mod unkown_type;

use dimension::run as dimension_run;
use event_box::run as event_box_run;
use game_attack::run as game_attack_run;
use payment::run as payment_run;
use plugin_system::run as plugin_system_run;
use print_it::run as print_it_run;
use storage::run as storage_run;
use ui::run as ui_run;
use unkown_type::run as unkown_type_run;

pub trait Module {
    fn try_run(&self);
}

struct Main {
    files: Vec<File>,
}

struct File {
    run_fn: Box<dyn Fn()>,
}

impl Module for File {
    fn try_run(&self) {
        (self.run_fn)();
    }
}

impl Main {
    fn run_all(&self) {
        self.files.iter().for_each(|file| {
            println!("------------");
            file.try_run();
            println!("------------");

            println!("\n");
        });
    }
}

pub fn run() {
    let main = Main {
        files: vec![
            File {
                run_fn: Box::new(dimension_run),
            },
            File {
                run_fn: Box::new(event_box_run),
            },
            File {
                run_fn: Box::new(game_attack_run),
            },
            File {
                run_fn: Box::new(payment_run),
            },
            File {
                run_fn: Box::new(plugin_system_run),
            },
            File {
                run_fn: Box::new(print_it_run),
            },
            File {
                run_fn: Box::new(unkown_type_run),
            },
            File {
                run_fn: Box::new(ui_run),
            },
            File {
                run_fn: Box::new(storage_run),
            },
        ],
    };

    main.run_all();
}
