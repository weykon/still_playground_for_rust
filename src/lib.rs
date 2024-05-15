mod box_rc_refcell_weak_mutex_rwlock_cell;
mod copy_derive_more;
mod dimension;
mod event_box;
mod exec_associated_type_fn;
mod game_attack;
mod gat;
mod if_dyn_trait_no_size_to_downcast_usage;
mod lateral_abstract;
mod life_and_rc_just_fire;
mod little_clearly;
mod onion;
mod payment;
mod plugin_system;
mod print_it;
mod ref_from_struct_by_lifetime;
mod relation_type;
mod storage;
mod ui;
mod unkown_type;
mod what_impl_for_trait_in_generic;
mod who_use_first;

use copy_derive_more::run as copy_derive_more_run;
use dimension::run as dimension_run;
use event_box::run as event_box_run;
use exec_associated_type_fn::run as exec_associated_type_fn_run;
use game_attack::run as game_attack_run;
use gat::run as GAT_run;
use if_dyn_trait_no_size_to_downcast_usage::run as if_dyn_trait_no_size_to_downcast_usage_run;
use lateral_abstract::run as lateral_abstract_run;
use life_and_rc_just_fire::run as life_and_rc_just_fire_run;
use little_clearly::run as little_clearly_run;
use onion::run as onion_run;
use payment::run as payment_run;
use plugin_system::run as plugin_system_run;
use print_it::run as print_it_run;
use ref_from_struct_by_lifetime::run as ref_from_struct_by_lifetime_run;
use relation_type::run as relation_type_run;
use storage::run as storage_run;
use ui::run as ui_run;
use unkown_type::run as unkown_type_run;
use what_impl_for_trait_in_generic::run as what_impl_for_trait_in_generic_run;
use who_use_first::run as who_use_first_run;

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
            File {
                run_fn: Box::new(relation_type_run),
            },
            File {
                run_fn: Box::new(GAT_run),
            },
            File {
                run_fn: Box::new(lateral_abstract_run),
            },
            File {
                run_fn: Box::new(if_dyn_trait_no_size_to_downcast_usage_run),
            },
            File {
                run_fn: Box::new(life_and_rc_just_fire_run),
            },
            File {
                run_fn: Box::new(who_use_first_run),
            },
            File {
                run_fn: Box::new(ref_from_struct_by_lifetime_run),
            },
            File {
                run_fn: Box::new(onion_run),
            },
            File {
                run_fn: Box::new(what_impl_for_trait_in_generic_run),
            },
            File {
                run_fn: Box::new(exec_associated_type_fn_run),
            },
            File {
                run_fn: Box::new(little_clearly_run),
            },
            File {
                run_fn: Box::new(copy_derive_more_run),
            },
        ],
    };

    main.run_all();
}
