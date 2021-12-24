#![forbid(unsafe_code)]
#![warn(
    // restriction lints
    clippy::clone_on_ref_ptr,
    clippy::create_dir,
    clippy::exit,
    clippy::filetype_is_file,
    clippy::fn_to_numeric_cast_any,
    clippy::get_unwrap,
    clippy::if_then_some_else_none,
    clippy::lossy_float_literal,
    clippy::multiple_inherent_impl,
    clippy::same_name_method,
    clippy::rc_buffer,
    clippy::rc_mutex,
    clippy::rest_pat_in_fully_bound_structs,
    clippy::unneeded_field_pattern,
    clippy::str_to_string,
    clippy::string_to_string,
    clippy::undocumented_unsafe_blocks,
    clippy::unnecessary_self_imports,

    // nursery lints
    clippy::debug_assert_with_mut_call,
    clippy::equatable_if_let,
    clippy::fallible_impl_from,
    clippy::option_if_let_else,
    clippy::path_buf_push_overwrite,
    clippy::use_self,
)]

use macroquad::prelude::*;
use egui_macroquad::egui;
//use tinyfiledialogs::{open_file_dialog, color_chooser_dialog, DefaultColorValue};
//use include_dir::{include_dir, Dir};
//use enumflags2::{bitflags, make_bitflags, BitFlags};

#[macroquad::main("BasicShapes")]
async fn main() {
    // File chooser
    //println!("{:?}", open_file_dialog("A cool-ass title", "", None));
    // Color picker
    //println!("{:?}", color_chooser_dialog(">:D", DefaultColorValue::Hex("#FF00FF")));
    // Mmmmm, binary-included directory!
    //println!("{:?}", DATA_DIR.get_file("pussy.txt").unwrap()); // 

    loop {
        clear_background(GRAY);

        egui_macroquad::ui(|ctx| { });

        egui_macroquad::draw();

        next_frame().await;
    }
}
