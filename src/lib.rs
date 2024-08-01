#![allow(warnings)]
mod add_digital;
mod avalible_outside_param_which_input;
mod box_rc_refcell_weak_mutex_rwlock_cell;
mod continue_gat;
mod copy_derive_more;
mod dimension;
mod downcast_ref_usage;
mod enum_dispatch;
mod enum_method_explain_weapon_sighting_telescope;
mod event_box;
mod exec_associated_type_fn;
mod fn_as_param;
mod game_attack;
mod gat;
mod gat_necessity;
mod good_on_any_type_to_downcast_usage;
mod higher_rank_trait_bound;
mod how_HKT_be_defined;
mod if_dyn_trait_no_size_to_downcast_usage;
mod json_parser;
mod lateral_abstract;
mod life_and_rc_just_fire;
mod little_clearly;
mod lots_generic;
mod more_gat;
mod onion;
mod paint_tree;
mod payment;
mod pls_easy_ast_token;
mod plugin_system;
mod print_it;
mod rc_refcell_come;
mod ref_and_use_anywhere;
mod ref_from_struct_by_lifetime;
mod relation_type;
mod rougelike_game;
mod self_box_using;
mod snake;
mod spiral_travel_tree;
mod still_at_GAT;
mod still_gat;
mod storage;
mod str_and_string;
mod tree_a_look;
mod tree_trait_as_generic;
mod try_option_param;
mod try_weak_weak;
mod ui;
mod unkown_type;
mod weak_detail;
mod what_impl_for_trait_in_generic;
mod who_use_first;
mod why_gat_rether_than_generic;
mod why_not_box;
mod cow_borrowed_need_remember;
mod keep_refcell_keypoint;
mod imagine_impl_any_t;
mod sandy;
mod again_any_asany;
mod impl_dyn_trait;

use add_digital::run as add_digital_run;
use avalible_outside_param_which_input::run as avalible_outside_param_which_input_run;
use continue_gat::run as continue_gat_run;
use copy_derive_more::run as copy_derive_more_run;
use dimension::run as dimension_run;
use downcast_ref_usage::run as downcast_ref_usage_run;
use enum_dispatch::run as enum_dispatch_run;
use enum_method_explain_weapon_sighting_telescope::run as enum_method_explain_weapon_sighting_telescope_run;
use event_box::run as event_box_run;
use exec_associated_type_fn::run as exec_associated_type_fn_run;
use fn_as_param::run as fn_as_param_run;
use game_attack::run as game_attack_run;
use gat::run as GAT_run;
use gat_necessity::run as gat_necessity_run;
use good_on_any_type_to_downcast_usage::run as good_on_any_type_to_downcast_usage_run;
use higher_rank_trait_bound::run as higher_rank_trait_bound_run;
use how_HKT_be_defined::run as how_HKT_be_defined_run;
use if_dyn_trait_no_size_to_downcast_usage::run as if_dyn_trait_no_size_to_downcast_usage_run;
use json_parser::run as json_parser_run;
use lateral_abstract::run as lateral_abstract_run;
use life_and_rc_just_fire::run as life_and_rc_just_fire_run;
use little_clearly::run as little_clearly_run;
use lots_generic::run as lots_generic_run;
use more_gat::run as more_gat_run;
use onion::run as onion_run;
use paint_tree::run as paint_tree_run;
use payment::run as payment_run;
use pls_easy_ast_token::run as pls_easy_ast_token_run;
use plugin_system::run as plugin_system_run;
use print_it::run as print_it_run;
use rc_refcell_come::run as rc_refcell_come_run;
use ref_and_use_anywhere::run as ref_and_use_anywhere_run;
use ref_from_struct_by_lifetime::run as ref_from_struct_by_lifetime_run;
use relation_type::run as relation_type_run;
use rougelike_game::run as rougelike_game_run;
use self_box_using::run as self_box_using_run;
use snake::run as snake_run;
use spiral_travel_tree::run as spiral_travel_tree_run;
use still_at_GAT::run as still_at_GAT_run;
use still_gat::run as still_gat_run;
use storage::run as storage_run;
use str_and_string::run as str_and_string_run;
use tree_a_look::run as tree_a_look_run;
use tree_trait_as_generic::run as tree_trait_as_generic_run;
use try_option_param::run as try_option_param_run;
use try_weak_weak::run as try_weak_weak_run;
use ui::run as ui_run;
use unkown_type::run as unkown_type_run;
use weak_detail::run as weak_detail_run;
use what_impl_for_trait_in_generic::run as what_impl_for_trait_in_generic_run;
use who_use_first::run as who_use_first_run;
use why_gat_rether_than_generic::run as why_gat_rether_than_generic_run;
use why_not_box::run as why_not_box_run;
use cow_borrowed_need_remember::run as cow_borrowed_need_remember_run;
use keep_refcell_keypoint::run as keep_refcell_keypoint_run;
use imagine_impl_any_t::run as imagine_impl_any_t_run;
use sandy::run as sandy_run;
use again_any_asany::run as again_any_asany_run;
use impl_dyn_trait::run as impl_dyn_trait_run;
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
        self.files.iter().rev().take(2).for_each(|file| {
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
            File {
                run_fn: Box::new(snake_run),
            },
            File {
                run_fn: Box::new(add_digital_run),
            },
            File {
                run_fn: Box::new(json_parser_run),
            },
            File {
                run_fn: Box::new(str_and_string_run),
            },
            File {
                run_fn: Box::new(pls_easy_ast_token_run),
            },
            File {
                run_fn: Box::new(tree_a_look_run),
            },
            File {
                run_fn: Box::new(rc_refcell_come_run),
            },
            File {
                run_fn: Box::new(continue_gat_run),
            },
            File {
                run_fn: Box::new(try_weak_weak_run),
            },
            File {
                run_fn: Box::new(weak_detail_run),
            },
            File {
                run_fn: Box::new(why_not_box_run),
            },
            File {
                run_fn: Box::new(spiral_travel_tree_run),
            },
            File {
                run_fn: Box::new(still_at_GAT_run),
            },
            File {
                run_fn: Box::new(paint_tree_run),
            },
            File {
                run_fn: Box::new(still_gat_run),
            },
            File {
                run_fn: Box::new(tree_trait_as_generic_run),
            },
            File {
                run_fn: Box::new(why_gat_rether_than_generic_run),
            },
            File {
                run_fn: Box::new(gat_necessity_run),
            },
            File {
                run_fn: Box::new(enum_method_explain_weapon_sighting_telescope_run),
            },
            File {
                run_fn: Box::new(enum_dispatch_run),
            },
            File {
                run_fn: Box::new(how_HKT_be_defined_run),
            },
            File {
                run_fn: Box::new(more_gat_run),
            },
            File {
                run_fn: Box::new(rougelike_game_run),
            },
            File {
                run_fn: Box::new(self_box_using_run),
            },
            File {
                run_fn: Box::new(fn_as_param_run),
            },
            File {
                run_fn: Box::new(downcast_ref_usage_run),
            },
            File {
                run_fn: Box::new(good_on_any_type_to_downcast_usage_run),
            },
            File {
                run_fn: Box::new(life_and_rc_just_fire_run),
            },
            File {
                run_fn: Box::new(avalible_outside_param_which_input_run),
            },
            File {
                run_fn: Box::new(ref_and_use_anywhere_run),
            },
            File {
                run_fn: Box::new(lots_generic_run),
            },
            File {
                run_fn: Box::new(try_option_param_run),
            },
            File {
                run_fn: Box::new(higher_rank_trait_bound_run),
            },
            File{
                run_fn: Box::new(cow_borrowed_need_remember_run),
            },
            File {
                run_fn: Box::new(keep_refcell_keypoint_run),
            },
            File {
                run_fn: Box::new(imagine_impl_any_t_run),
            },
            File {
                run_fn: Box::new(sandy_run),
            },
            File {
                run_fn: Box::new(again_any_asany_run),
            },
            File { 
                run_fn: Box::new(impl_dyn_trait_run),
            }
        ],
    };

    main.run_all();
}
